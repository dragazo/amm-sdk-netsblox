#![forbid(unsafe_code)]
#![no_std]

#[macro_use]
extern crate alloc;

use core::fmt::Write as _;
use core::iter;

use alloc::vec::Vec;
use alloc::collections::{BTreeMap, BTreeSet};
use alloc::string::String;

pub use amm_sdk; // re-export for lib users

use amm_sdk::Composition;
use amm_sdk::note::{Note, DurationType, Duration, Accidental};
use amm_sdk::context::{Key, Tempo};
use amm_sdk::modification::{PhraseModificationType, NoteModificationType, SectionModificationType, DirectionType, NoteModification, ChordModificationType};
use amm_sdk::structure::{Part, Section, Staff, PartContent, SectionContent, StaffContent, ChordContent, Phrase, PhraseContent};

fn xml_escape(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    for c in input.chars() {
        match c {
            '&' => result.push_str("&amp;"),
            '<' => result.push_str("&lt;"),
            '>' => result.push_str("&gt;"),
            '\'' => result.push_str("&apos;"),
            '"' => result.push_str("&quot;"),
            '\n' => result.push_str("&#xD;"),
            _ => result.push(c),
        }
    }
    result
}
fn quarter_note_tempo(tempo: &Tempo) -> f64 {
    tempo.beats_per_minute as f64 * (tempo.base_note.value() / Duration::new(DurationType::Quarter, 0).value())
}

#[derive(Debug)]
pub enum TranslateError {
    CyclicStructure,
    UnsupportedDuration { duration: Duration },
    UnsupportedTuplet { num_beats: u8, into_beats: u8 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Mod {
    Accent, Staccato, TurnUpper, TurnLower,
}
#[derive(Default)]
struct Modifiers {
    stack: Vec<Vec<Mod>>,
    active: BTreeSet<Mod>,
}

macro_rules! check_modifiers_invariants {
    ($self:ident) => {{
        debug_assert!($self.stack.iter().all(|x| !x.is_empty()));
        debug_assert!($self.stack.iter().map(|x| x.len()).sum::<usize>() == $self.active.len());
        debug_assert!($self.stack.iter().flat_map(|x| x.iter().copied()).collect::<BTreeSet<_>>() == $self.active);
    }};
}

impl Modifiers {
    fn set(&mut self, new_active: &BTreeSet<Mod>, output: &mut String) {
        check_modifiers_invariants!(self);

        while !self.active.is_subset(new_active) {
            for x in self.stack.pop().unwrap() {
                self.active.remove(&x);
            }
            write!(output, r#"</script></block>"#).unwrap();
        }

        let new = (new_active - &self.active).into_iter().collect::<Vec<_>>();

        if !new.is_empty() {
            write!(output, r#"<block s="noteMod"><list>"#).unwrap();
            for x in new.iter() {
                self.active.insert(*x);
                write!(output, r#"<l><option>{x:?}</option></l>"#).unwrap();
            }
            write!(output, r#"</list><script>"#).unwrap();

            self.stack.push(new);
        }

        check_modifiers_invariants!(self);
    }
    fn unwind_point(&self) -> usize {
        self.stack.len()
    }
    fn unwind_to(&mut self, point: usize, output: &mut String) {
        check_modifiers_invariants!(self);

        while self.stack.len() > point {
            for x in self.stack.pop().unwrap() {
                self.active.remove(&x);
            }
            write!(output, r#"</script></block>"#).unwrap();
        }

        check_modifiers_invariants!(self);
    }
}

struct Context {
    modifiers: Modifiers,
    sections: BTreeSet<*const Section>,
    staffs: BTreeSet<*const Staff>,
    phrases: BTreeSet<*const Phrase>,
    starting_key: Key,
    starting_tempo: Tempo,
    blocks: BTreeMap<String, String>,
}

fn half_duration_type(duration_type: DurationType) -> Option<DurationType> {
    match duration_type {
        DurationType::Maxima => Some(DurationType::Long),
        DurationType::Long => Some(DurationType::Breve),
        DurationType::Breve => Some(DurationType::Whole),
        DurationType::Whole => Some(DurationType::Half),
        DurationType::Half => Some(DurationType::Quarter),
        DurationType::Quarter => Some(DurationType::Eighth),
        DurationType::Eighth => Some(DurationType::Sixteenth),
        DurationType::Sixteenth => Some(DurationType::ThirtySecond),
        DurationType::ThirtySecond => Some(DurationType::SixtyFourth),
        DurationType::SixtyFourth => Some(DurationType::OneHundredTwentyEighth),
        DurationType::OneHundredTwentyEighth => Some(DurationType::TwoHundredFiftySixth),
        DurationType::TwoHundredFiftySixth => Some(DurationType::FiveHundredTwelfth),
        DurationType::FiveHundredTwelfth => Some(DurationType::OneThousandTwentyFourth),
        DurationType::OneThousandTwentyFourth => Some(DurationType::TwoThousandFortyEighth),
        DurationType::TwoThousandFortyEighth => None,
    }
}
fn parse_duration(duration: Duration) -> Result<String, TranslateError> {
    let dots = match duration.dots {
        0 => "",
        1 => "Dotted",
        2 => "DottedDotted",
        x => {
            let mut res = String::from(r#"<block s="tieDuration"><list>"#);
            let mut t = duration.value;
            for _ in 2..x {
                res += &parse_duration(Duration::new(t, 0)).map_err(|_| TranslateError::UnsupportedDuration { duration })?;
                t = half_duration_type(t).ok_or_else(|| TranslateError::UnsupportedDuration { duration })?;
            }
            res += &parse_duration(Duration::new(t, 2)).map_err(|_| TranslateError::UnsupportedDuration { duration })?;
            res += "</list></block>";
            return Ok(res);
        }
    };
    Ok(match duration.value {
        DurationType::Maxima => format!(r#"<block s="tieDuration"><list><l>{dots}Whole</l><l>{dots}Whole</l><l>{dots}Whole</l><l>{dots}Whole</l><l>{dots}Whole</l><l>{dots}Whole</l><l>{dots}Whole</l><l>{dots}Whole</l></list></block>"#),
        DurationType::Long => format!(r#"<block s="tieDuration"><list><l>{dots}Whole</l><l>{dots}Whole</l><l>{dots}Whole</l><l>{dots}Whole</l></list></block>"#),
        DurationType::Breve => format!(r#"<block s="tieDuration"><list><l>{dots}Whole</l><l>{dots}Whole</l></list></block>"#),
        DurationType::Whole => format!("<l>{dots}Whole</l>"),
        DurationType::Half => format!("<l>{dots}Half</l>"),
        DurationType::Quarter => format!("<l>{dots}Quarter</l>"),
        DurationType::Eighth => format!("<l>{dots}Eighth</l>"),
        DurationType::Sixteenth => format!("<l>{dots}Sixteenth</l>"),
        DurationType::ThirtySecond => format!("<l>{dots}ThirtySecond</l>"),
        DurationType::SixtyFourth => format!("<l>{dots}SixtyFourth</l>"),
        _ => return Err(TranslateError::UnsupportedDuration { duration }),
    })
}
fn translate_chord(raw_notes: &[Note], raw_mods: &[ChordModificationType], output: &mut String, context: &mut Context) -> Result<(), TranslateError> {
    let raw_mods = raw_mods.iter().flat_map(NoteModification::from_chord_modification).map(|x| x.r#type).collect::<Vec<_>>();

    for m in raw_notes.iter().flat_map(|n| n.iter_modifications()).map(|m| &m.r#type).chain(&raw_mods) {
        match m {
            NoteModificationType::Dynamic { dynamic } => write!(output, r#"<block s="setAudioEffect"><l>Volume</l><l>{}</l></block>"#, 100.0 * dynamic.value()).unwrap(),
            _ => (),
        }
    }

    // in the future, beatblox will support grace notes - but for now, just ignore them
    let raw_notes = raw_notes.iter().filter(|x| !x.iter_modifications().any(|m| matches!(m.r#type, NoteModificationType::Grace { .. })));

    let (notes, shortest_duration) = match raw_notes.clone().map(|x| x.duration).reduce(|a, b| if a.value() <= b.value() { a } else { b }) {
        Some(x) => (raw_notes.filter(|x| !x.is_rest()), parse_duration(x)?),
        None => return Ok(()),
    };

    if notes.clone().next().is_some() {
        let mut notes_xml = String::new();
        let mut durations_xml = vec![];
        for note in notes.clone() {
            let accidental = match note.accidental {
                Accidental::None => "",
                Accidental::Natural => "n",
                Accidental::Sharp => "s",
                Accidental::DoubleSharp => "ss",
                Accidental::Flat => "b",
                Accidental::DoubleFlat => "bb",
            };
            write!(notes_xml, "<l>{pitch}{accidental}</l>", pitch = note.pitch).unwrap();
            durations_xml.push(parse_duration(note.duration)?);
        }
        if !durations_xml.contains(&shortest_duration) {
            write!(notes_xml, "<l>rest</l>").unwrap();
            durations_xml.push(shortest_duration);
        }
        let durations_xml = if durations_xml.iter().all(|x| *x == durations_xml[0]) { durations_xml.into_iter().next().unwrap() } else { format!(r#"<block s="reportNewList"><list>{}</list></block>"#, durations_xml.join("")) };

        let mods = notes.flat_map(|n| n.iter_modifications().map(|x| &x.r#type)).chain(&raw_mods).flat_map(|m| Some(match &m {
            NoteModificationType::Accent | NoteModificationType::SoftAccent => Mod::Accent,
            NoteModificationType::Staccato | NoteModificationType::Staccatissimo => Mod::Staccato,
            NoteModificationType::Turn { upper, delayed: _, vertical: _ } => if *upper { Mod::TurnUpper } else { Mod::TurnLower },
            _ => return None,
        })).collect();
        context.modifiers.set(&mods, output);

        write!(output, r#"<block s="playNotes">{durations_xml}<list>{notes_xml}</list></block>"#).unwrap();
    } else {
        write!(output, r#"<block s="rest">{shortest_duration}</block>"#).unwrap();
    }

    Ok(())
}
fn translate_phrase(phrase: &Phrase, output: &mut String, context: &mut Context) -> Result<(), TranslateError> {
    if !context.phrases.insert(phrase as *const _) {
        return Err(TranslateError::CyclicStructure);
    }

    let mut tuplet_mod = None;
    for modification in phrase.iter_modifications() {
        match modification.r#type {
            PhraseModificationType::Tuplet { num_beats, into_beats } => match (num_beats, into_beats) {
                (3, 2) => tuplet_mod = Some("Tuplet 3:2"),
                (5, 4) => tuplet_mod = Some("Tuplet 5:4"),
                (6, 4) => tuplet_mod = Some("Tuplet 6:4"),
                (7, 4) => tuplet_mod = Some("Tuplet 7:4"),
                _ => return Err(TranslateError::UnsupportedTuplet { num_beats, into_beats }),
            }
            _ => (),
        }
    }

    let unwind_point = context.modifiers.unwind_point();
    if let Some(tuplet_mod) = tuplet_mod {
        write!(output, r#"<block s="noteMod"><list><l><option>{tuplet_mod}</option></l></list><script>"#).unwrap();
    }

    for content in phrase.iter() {
        match content {
            PhraseContent::Note(note) => translate_chord(&[note.clone()], &[], output, context)?,
            PhraseContent::Chord(chord) => translate_chord(&chord.iter().map(|x| match x { ChordContent::Note(note) => note.clone() }).collect::<Vec<_>>(), &chord.iter_modifications().map(|x| x.r#type).collect::<Vec<_>>(), output, context)?,
            PhraseContent::Phrase(sub_phrase) => translate_phrase(sub_phrase, output, context)?,
            PhraseContent::MultiVoice(_) => (),
        }
    }

    if tuplet_mod.is_some() {
        write!(output, r#"</script></block>"#).unwrap();
        context.modifiers.unwind_to(unwind_point, output);
    }

    assert!(context.phrases.remove(&(phrase as *const _)));
    Ok(())
}
fn translate_staff(staff: &Staff, output: &mut String, context: &mut Context) -> Result<(), TranslateError> {
    if !context.staffs.insert(staff as *const _) {
        return Err(TranslateError::CyclicStructure);
    }

    for content in staff.iter() {
        match content {
            StaffContent::Note(note) => translate_chord(&[note.clone()], &[], output, context)?,
            StaffContent::Chord(chord) => translate_chord(&chord.iter().map(|x| match x { ChordContent::Note(note) => note.clone() }).collect::<Vec<_>>(), &chord.iter_modifications().map(|x| x.r#type).collect::<Vec<_>>(), output, context)?,
            StaffContent::Phrase(phrase) => translate_phrase(phrase, output, context)?,
            StaffContent::Direction(direction) => match &direction.r#type {
                DirectionType::KeyChange { key } => write!(output, r#"<block s="setKey"><l>{key_sig:?}{key_mode:?}</l></block>"#, key_sig = key.signature, key_mode = key.mode).unwrap(),
                _ => (),
            }
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
        match &modification.r#type {
            SectionModificationType::Repeat { num_times } => repetitions += *num_times as usize,
            SectionModificationType::TempoExplicit { tempo } => write!(output, r#"<block s="setBPM"><l>{tempo}</l></block>"#, tempo = quarter_note_tempo(tempo)).unwrap(),
            SectionModificationType::TempoImplicit { tempo } => write!(output, r#"<block s="setBPM"><l>{tempo}</l></block>"#, tempo = tempo.value()).unwrap(),
            _ => (),
        }
    }

    if repetitions != 1 {
        write!(output, r#"<block s="doRepeat"><l>{repetitions}</l><script>"#).unwrap();
    }

    for content in section.iter() {
        match content {
            SectionContent::Staff(staff) => translate_staff(staff, output, context)?,
            SectionContent::Section(section) => translate_section(section, output, context)?,
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
    let instrument = match part.get_name().to_lowercase().as_str() {
        x if x.contains("synth") => "Synthesizer",
        x if x.contains("bassoon") => "Bassoon",
        x if x.contains("bass") => "Electric Bass",
        x if x.contains("cello") => "Cello",
        x if x.contains("guitar") => match x {
            x if x.contains("elec") => "Electric Guitar",
            x if x.contains("nylon") => "Nylon Guitar",
            _ => "Acoustic Guitar",
        }
        x if x.contains("harp") => "Harp",
        x if x.contains("organ") => "Pipe Organ",
        x if x.contains("violin") => "Violin",
        _ => "Grand Piano",
    };

    write!(output, r#"<sprite name="{name}" x="0" y="0" heading="90" scale="1" volume="100" pan="0" rotation="1" draggable="true" costume="0" color="80,80,80,1" pen="tip"><costumes><list struct="atomic"></list></costumes><sounds><list struct="atomic"></list></sounds><blocks></blocks><variables></variables><scripts>"#).unwrap();

    write!(output, r#"<script x="0" y="0"><block s="receiveGo"></block>"#).unwrap();
    write!(output, r#"<block s="setInstrument"><l>{instrument}</l></block>"#).unwrap();
    write!(output, r#"<block s="setBPM"><l>{tempo}</l></block>"#, tempo = quarter_note_tempo(&context.starting_tempo)).unwrap();
    write!(output, r#"<block s="setKey"><l>{key_sig:?}{key_mode:?}</l></block>"#, key_sig = context.starting_key.signature, key_mode = context.starting_key.mode).unwrap();

    for content in part.iter() {
        debug_assert!(context.modifiers.stack.is_empty() && context.modifiers.active.is_empty());
        match content {
            PartContent::Section(section) => {
                let block_name = iter::once(String::new()).chain((2usize..).map(|x| format!(" {x}"))).map(|x| format!("{}{x}", section.get_name())).find(|x| !context.blocks.contains_key(x)).unwrap();
                let mut block_def = format!(r#"<block-definition s={block_name:?} type="command" category="music"><inputs></inputs><script>"#);
                translate_section(section, &mut block_def, context)?;
                context.modifiers.set(&Default::default(), &mut block_def);
                write!(block_def, "</script></block-definition>").unwrap();
                write!(output, r#"<custom-block s={block_name:?}></custom-block>"#).unwrap();
                context.blocks.insert(block_name, block_def);
            }
        }
    }

    write!(output, r#"</script></scripts></sprite>"#).unwrap();
    Ok(())
}
pub fn translate(composition: &Composition) -> Result<String, TranslateError> {
    let composition = composition.restructure_staves_as_parts().flatten();
    let title = xml_escape(composition.get_title());
    let tempo = quarter_note_tempo(composition.get_tempo());

    let stringify_list = |x: &[String]| if !x.is_empty() { x.join(", ") } else { "N/A".into() };
    let notes = xml_escape(&format!("title: {title}\ncomposers: {composers}\nlyricists: {lyricists}\narrangers: {arrangers}\npublisher: {publisher}\ncopyright: {copyright}\n\ntempo: {tempo}\ntime signature: {time_signature}\nkey: {key_sig:?}{key_mode:?}",
        title = composition.get_title(),
        composers = stringify_list(composition.get_composers()),
        lyricists = stringify_list(composition.get_lyricists()),
        arrangers = stringify_list(composition.get_arrangers()),
        publisher = stringify_list(composition.get_publisher().as_slice()),
        copyright = stringify_list(composition.get_copyright().as_slice()),
        time_signature = composition.get_starting_time_signature(),
        key_sig = composition.get_starting_key().signature,
        key_mode = composition.get_starting_key().mode,
    ));

    let mut res = String::new();
    write!(res, r#"<room name="{title}"><role name="myRole"><project name="myRole"><notes>{notes}</notes><stage name="Stage" width="480" height="360" costume="0" color="255,255,255,1" tempo="{tempo}" threadsafe="false" penlog="false" volume="100" pan="0" lines="round" ternary="false" hyperops="true" codify="false" inheritance="false" sublistIDs="false" scheduled="false"><costumes><list struct="atomic"></list></costumes><sounds><list struct="atomic"></list></sounds><variables></variables><blocks></blocks><messageTypes><messageType><name>message</name><fields><field>msg</field></fields></messageType></messageTypes><scripts></scripts><sprites>"#).unwrap();

    let mut context = Context {
        modifiers: <_>::default(),
        sections: <_>::default(),
        phrases: <_>::default(),
        staffs: <_>::default(),
        starting_key: *composition.get_starting_key(),
        starting_tempo: *composition.get_tempo(),
        blocks: <_>::default(),
    };
    for part in composition.iter() {
        translate_part(part, &mut res, &mut context)?;
    }

    write!(res, r#"</sprites></stage><blocks>"#).unwrap();
    for block_def in context.blocks.values() {
        res += block_def.as_str();
    }
    write!(res, r#"</blocks><variables></variables></project><media name="myRole"></media></role></room>"#).unwrap();

    Ok(res)
}
