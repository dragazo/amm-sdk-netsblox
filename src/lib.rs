#![forbid(unsafe_code)]
#![no_std]

#[macro_use]
extern crate alloc;

use core::fmt::Write as _;

use alloc::vec::Vec;
use alloc::collections::BTreeSet;
use alloc::string::{String, ToString};

use amm::{
    Composition, Part, Section, Staff, Note, PartContent, SectionContent, StaffContent, ChordContent, DurationType, SectionModificationType,
    NoteModificationType, Dynamic, DynamicMarking,
};

fn xml_escape(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    for c in input.chars() {
        match c {
            '&' => result.push_str("&amp;"),
            '<' => result.push_str("&lt;"),
            '>' => result.push_str("&gt;"),
            '\'' => result.push_str("&apos;"),
            '"' => result.push_str("&quot;"),
            o => result.push(o),
        }
    }
    result.into()
}

#[derive(Debug)]
pub enum TranslateError {
    CyclicStructure,
    UnsupportedDuration { duration: String }, // actual Duration isn't Debug
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Mod {
    Piano, Forte, Accent, Staccato,
}
#[derive(Default)]
struct Modifiers {
    stack: Vec<Mod>,
    active: BTreeSet<Mod>,
}
impl Modifiers {
    fn set(&mut self, new_active: &BTreeSet<Mod>, output: &mut String) {
        debug_assert!(self.stack.len() == self.active.len() && self.stack.iter().copied().collect::<BTreeSet<_>>() == self.active);

        while !self.active.is_subset(&new_active) {
            self.active.remove(&self.stack.pop().unwrap());
            write!(output, r#"</script></block>"#).unwrap();
        }

        for &new in new_active {
            if self.active.insert(new) {
                self.stack.push(new);
                write!(output, r#"<block s="noteModifierC"><l>{new:?}</l><script>"#).unwrap();
            }
        }
    }
}

#[derive(Default)]
struct Context {
    modifiers: Modifiers,
    sections: BTreeSet<*const Section>,
    staffs: BTreeSet<*const Staff>,
}

fn translate_chord(notes: &[Note], output: &mut String, context: &mut Context) -> Result<(), TranslateError> {
    let duration = match notes.iter().map(|t| t.duration).reduce(|a, b| if a.value() <= b.value() { a } else { b }) {
        Some(x) => x,
        None => return Ok(()),
    };
    let duration_value = match duration.value {
        DurationType::Whole => "Whole",
        DurationType::Half => "Half",
        DurationType::Quarter => "Quarter",
        DurationType::Eighth => "Eighth",
        DurationType::Sixteenth => "Sixteenth",
        DurationType::ThirtySecond => "ThirtySecond",
        DurationType::SixtyFourth => "SixtyFourth",
        _ => return Err(TranslateError::UnsupportedDuration { duration: duration.to_string() }),
    };
    let duration_dots = match duration.dots {
        0 => "",
        1 => "Dotted",
        2 => "DottedDotted",
        _ => return Err(TranslateError::UnsupportedDuration { duration: duration.to_string() }),
    };

    let mut raw_notes_xml = String::new();
    for note in notes.iter() {
        write!(raw_notes_xml, "<l>{}</l>", note.pitch).unwrap();
    }
    let notes_xml = if notes.len() == 1 { raw_notes_xml } else { format!(r#"<block s="reportNewList"><list>{raw_notes_xml}</list></block>"#) };

    let mods = notes.iter().map(|n| n.iter_modifications().flat_map(|m| Some(match m.borrow().get_modification() {
        NoteModificationType::Accent | NoteModificationType::SoftAccent => Mod::Accent,
        NoteModificationType::Staccato | NoteModificationType::Staccatissimo => Mod::Staccato,
        NoteModificationType::Dynamic { dynamic: Dynamic { marking: DynamicMarking::Forte | DynamicMarking::MezzoForte, repetitions: _ } } => Mod::Forte,
        NoteModificationType::Dynamic { dynamic: Dynamic { marking: DynamicMarking::Piano | DynamicMarking::MezzoPiano, repetitions: _ } } => Mod::Piano,
        _ => return None,
    })).collect::<BTreeSet<_>>()).reduce(|a, b| &a | &b).unwrap();
    context.modifiers.set(&mods, output);

    write!(output, r#"<block s="playNote">{notes_xml}<l>{duration_value}</l><l>{duration_dots}</l></block>"#).unwrap();

    Ok(())
}
fn translate_staff(staff: &Staff, output: &mut String, context: &mut Context) -> Result<(), TranslateError> {
    if !context.staffs.insert(staff as *const _) {
        return Err(TranslateError::CyclicStructure);
    }

    for content in staff.iter() {
        match content {
            StaffContent::Note(note) => translate_chord(&[note.borrow().clone()], output, context)?,
            StaffContent::Chord(chord) => translate_chord(&chord.borrow().iter().map(|x| match x { ChordContent::Note(note) => note.borrow().clone() }).collect::<Vec<_>>(), output, context)?,
            StaffContent::Phrase(_) => (),
            StaffContent::Direction(_) => (),
            StaffContent::MultiVoice(_) => (),
        }
    }

    assert!(context.staffs.remove(&(staff as *const _)));
    Ok(())
}
fn translate_section(section: &Section, output: &mut String, context: &mut Context) -> Result<(), TranslateError> {
    if !context.sections.insert(section as *const _) {
        return Err(TranslateError::CyclicStructure);
    }

    let mut repetitions = 1;
    for modification in section.iter_modifications() {
        match modification.borrow().get_modification() {
            SectionModificationType::Repeat { num_times } => repetitions += *num_times as usize,
            _ => (),
        }
    }

    if repetitions != 1 {
        write!(output, r#"<block s="doRepeat"><l>{repetitions}</l><script>"#).unwrap();
    }

    for content in section.iter() {
        match content {
            SectionContent::Staff(staff) => translate_staff(&*staff.borrow(), output, context)?,
            SectionContent::Section(section) => translate_section(&*section.borrow(), output, context)?,
        }
    }

    if repetitions != 1 {
        write!(output, r#"</script></block>"#).unwrap();
    }

    assert!(context.sections.remove(&(section as *const _)));
    Ok(())
}
fn translate_part(part: &Part, output: &mut String, context: &mut Context) -> Result<(), TranslateError> {
    let name = xml_escape(part.get_name());

    write!(output, r#"<sprite name="{name}" x="0" y="0" heading="90" scale="1" volume="100" pan="0" rotation="1" draggable="true" costume="0" color="80,80,80,1" pen="tip"><costumes><list struct="atomic"></list></costumes><sounds><list struct="atomic"></list></sounds><blocks></blocks><variables></variables><scripts>"#).unwrap();

    for (i, content) in part.iter().enumerate() {
        let (x, y) = (i as f64 * 300.0, 0.0);
        write!(output, r#"<script x="{x}" y="{y}"><block s="receiveGo"></block>"#).unwrap();

        debug_assert!(context.modifiers.stack.is_empty() && context.modifiers.active.is_empty());
        match content {
            PartContent::Section(section) => translate_section(&*section.borrow(), output, context)?,
        }
        context.modifiers.set(&Default::default(), output);

        write!(output, r#"</script>"#).unwrap();
    }

    write!(output, r#"</scripts></sprite>"#).unwrap();
    Ok(())
}
pub fn translate(composition: &Composition) -> Result<String, TranslateError> {
    let composition = composition.restructure_staves_as_parts().flatten();
    let title = xml_escape(composition.get_title());
    let tempo = composition.get_tempo().beats_per_minute;

    let mut res = String::new();
    write!(res, r#"<room name="{title}"><role name="myRole"><project name="myRole"><stage name="Stage" width="480" height="360" costume="0" color="255,255,255,1" tempo="{tempo}" threadsafe="false" penlog="false" volume="100" pan="0" lines="round" ternary="false" hyperops="true" codify="false" inheritance="false" sublistIDs="false" scheduled="false"><costumes><list struct="atomic"></list></costumes><sounds><list struct="atomic"></list></sounds><variables></variables><blocks></blocks><messageTypes><messageType><name>message</name><fields><field>msg</field></fields></messageType></messageTypes><scripts></scripts><sprites>"#).unwrap();

    let mut context = Context::default();
    for part in composition.iter() {
        translate_part(part, &mut res, &mut context)?;
    }

    write!(res, r#"</sprites></stage><blocks></blocks><variables></variables></project><media name="myRole"></media></role></room>"#).unwrap();

    Ok(res)
}
