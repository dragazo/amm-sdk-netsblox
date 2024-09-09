use amm::{Composition, Pitch, Duration, DurationType, PitchName, SectionModificationType, NoteModificationType, Dynamic, DynamicMarking, PhraseModificationType};

use amm_sdk_netsblox::*;

#[test]
fn test_notes() {
    let composition = {
        let mut composition = Composition::new("untitled", None, None, None);
        let part = composition.add_part("part0");
        let section = part.add_section("sec0");
        let mut section = section.borrow_mut();
        let staff = section.add_staff("staff0", None, None, None);
        let mut staff = staff.borrow_mut();

        staff.add_note(Pitch::new(PitchName::D, 1), Duration::new(DurationType::Whole, 0), None);
        staff.add_note(Pitch::new(PitchName::E, 2), Duration::new(DurationType::Half, 0), None);
        staff.add_note(Pitch::new(PitchName::A, 3), Duration::new(DurationType::Quarter, 0), None);
        staff.add_note(Pitch::new(PitchName::D, 4), Duration::new(DurationType::Eighth, 0), None);
        staff.add_note(Pitch::new(PitchName::B, 3), Duration::new(DurationType::Sixteenth, 0), None);
        staff.add_note(Pitch::new(PitchName::E, 2), Duration::new(DurationType::ThirtySecond, 0), None);
        staff.add_note(Pitch::new(PitchName::E, 1), Duration::new(DurationType::SixtyFourth, 0), None);

        staff.add_note(Pitch::new(PitchName::D, 1), Duration::new(DurationType::Whole, 1), None);
        staff.add_note(Pitch::new(PitchName::E, 2), Duration::new(DurationType::Half, 1), None);
        staff.add_note(Pitch::new(PitchName::A, 3), Duration::new(DurationType::Quarter, 1), None);
        staff.add_note(Pitch::new(PitchName::D, 4), Duration::new(DurationType::Eighth, 1), None);
        staff.add_note(Pitch::new(PitchName::B, 3), Duration::new(DurationType::Sixteenth, 1), None);
        staff.add_note(Pitch::new(PitchName::E, 2), Duration::new(DurationType::ThirtySecond, 1), None);
        staff.add_note(Pitch::new(PitchName::E, 1), Duration::new(DurationType::SixtyFourth, 1), None);

        staff.add_note(Pitch::new(PitchName::D, 1), Duration::new(DurationType::Whole, 2), None);
        staff.add_note(Pitch::new(PitchName::E, 2), Duration::new(DurationType::Half, 2), None);
        staff.add_note(Pitch::new(PitchName::A, 3), Duration::new(DurationType::Quarter, 2), None);
        staff.add_note(Pitch::new(PitchName::D, 4), Duration::new(DurationType::Eighth, 2), None);
        staff.add_note(Pitch::new(PitchName::B, 3), Duration::new(DurationType::Sixteenth, 2), None);
        staff.add_note(Pitch::new(PitchName::E, 2), Duration::new(DurationType::ThirtySecond, 2), None);
        staff.add_note(Pitch::new(PitchName::E, 1), Duration::new(DurationType::SixtyFourth, 2), None);

        composition
    };

    assert_eq!(translate(&composition).unwrap(), include_str!("projects/notes.xml"));
}

#[test]
fn test_chords() {
    let composition = {
        let mut composition = Composition::new("untitled", None, None, None);
        let part = composition.add_part("part0");
        let section = part.add_section("sec0");
        let mut section = section.borrow_mut();
        let staff = section.add_staff("staff0", None, None, None);
        let mut staff = staff.borrow_mut();

        let _ = staff.add_chord();

        let chord = staff.add_chord();
        let mut chord = chord.borrow_mut();
        chord.add_note(Pitch::new(PitchName::B, 2), Duration::new(DurationType::Eighth, 2), None);

        let chord = staff.add_chord();
        let mut chord = chord.borrow_mut();
        chord.add_note(Pitch::new(PitchName::B, 2), Duration::new(DurationType::Half, 1), None);
        chord.add_note(Pitch::new(PitchName::E, 4), Duration::new(DurationType::Half, 1), None);

        let chord = staff.add_chord();
        let mut chord = chord.borrow_mut();
        chord.add_note(Pitch::new(PitchName::D, 4), Duration::new(DurationType::Quarter, 0), None);
        chord.add_note(Pitch::new(PitchName::E, 2), Duration::new(DurationType::Quarter, 0), None);
        chord.add_note(Pitch::new(PitchName::F, 3), Duration::new(DurationType::Quarter, 0), None);

        composition
    };

    assert_eq!(translate(&composition).unwrap(), include_str!("projects/chords.xml"));
}

#[test]
fn test_note_mods() {
    let composition = {
        let mut composition = Composition::new("untitled", None, None, None);
        let part = composition.add_part("part0");
        let section = part.add_section("sec0");
        let mut section = section.borrow_mut();
        let staff = section.add_staff("staff0", None, None, None);
        let mut staff = staff.borrow_mut();

        let _ = staff.add_note(Pitch::new(PitchName::C, 4), Duration::new(DurationType::Quarter, 0), None);

        let note = staff.add_note(Pitch::new(PitchName::E, 4), Duration::new(DurationType::Quarter, 0), None);
        note.borrow_mut().add_modification(NoteModificationType::Dynamic { dynamic: Dynamic::new(DynamicMarking::Piano, 0) });

        let _ = staff.add_note(Pitch::new(PitchName::C, 3), Duration::new(DurationType::Quarter, 0), None);

        let note = staff.add_note(Pitch::new(PitchName::D, 3), Duration::new(DurationType::Quarter, 0), None);
        note.borrow_mut().add_modification(NoteModificationType::Dynamic { dynamic: Dynamic::new(DynamicMarking::Forte, 0) });

        let note = staff.add_note(Pitch::new(PitchName::E, 3), Duration::new(DurationType::Quarter, 0), None);
        note.borrow_mut().add_modification(NoteModificationType::Dynamic { dynamic: Dynamic::new(DynamicMarking::Forte, 0) });

        let _ = staff.add_note(Pitch::new(PitchName::C, 3), Duration::new(DurationType::Quarter, 0), None);

        let note = staff.add_note(Pitch::new(PitchName::E, 4), Duration::new(DurationType::Quarter, 0), None);
        note.borrow_mut().add_modification(NoteModificationType::Turn { upper: true, delayed: false, vertical: false });

        let note = staff.add_note(Pitch::new(PitchName::E, 4), Duration::new(DurationType::Quarter, 0), None);
        note.borrow_mut().add_modification(NoteModificationType::Turn { upper: false, delayed: false, vertical: false });

        let _ = staff.add_note(Pitch::new(PitchName::C, 3), Duration::new(DurationType::Quarter, 0), None);

        let note = staff.add_note(Pitch::new(PitchName::D, 3), Duration::new(DurationType::Quarter, 0), None);
        note.borrow_mut().add_modification(NoteModificationType::Dynamic { dynamic: Dynamic::new(DynamicMarking::Piano, 0) });
        note.borrow_mut().add_modification(NoteModificationType::Accent);

        let note = staff.add_note(Pitch::new(PitchName::E, 3), Duration::new(DurationType::Quarter, 0), None);
        note.borrow_mut().add_modification(NoteModificationType::Accent);
        note.borrow_mut().add_modification(NoteModificationType::Dynamic { dynamic: Dynamic::new(DynamicMarking::Piano, 0) });

        let note = staff.add_note(Pitch::new(PitchName::D, 3), Duration::new(DurationType::Quarter, 0), None);
        note.borrow_mut().add_modification(NoteModificationType::Accent);
        note.borrow_mut().add_modification(NoteModificationType::Dynamic { dynamic: Dynamic::new(DynamicMarking::Piano, 0) });
        note.borrow_mut().add_modification(NoteModificationType::Staccato);

        let note = staff.add_note(Pitch::new(PitchName::E, 3), Duration::new(DurationType::Quarter, 0), None);
        note.borrow_mut().add_modification(NoteModificationType::Staccato);
        note.borrow_mut().add_modification(NoteModificationType::Accent);
        note.borrow_mut().add_modification(NoteModificationType::Dynamic { dynamic: Dynamic::new(DynamicMarking::Piano, 0) });

        let note = staff.add_note(Pitch::new(PitchName::C, 2), Duration::new(DurationType::Quarter, 0), None);
        note.borrow_mut().add_modification(NoteModificationType::Dynamic { dynamic: Dynamic::new(DynamicMarking::Piano, 0) });
        note.borrow_mut().add_modification(NoteModificationType::Accent);

        let note = staff.add_note(Pitch::new(PitchName::G, 4), Duration::new(DurationType::Quarter, 0), None);
        note.borrow_mut().add_modification(NoteModificationType::Accent);
        note.borrow_mut().add_modification(NoteModificationType::Dynamic { dynamic: Dynamic::new(DynamicMarking::Piano, 0) });

        composition
    };

    assert_eq!(translate(&composition).unwrap(), include_str!("projects/note-mods.xml"));
}

#[test]
fn test_triplets() {
    let composition = {
        let mut composition = Composition::new("untitled", None, None, None);
        let part = composition.add_part("part0");
        let section = part.add_section("sec0");
        let mut section = section.borrow_mut();
        let staff = section.add_staff("staff0", None, None, None);
        let mut staff = staff.borrow_mut();

        let _ = staff.add_note(Pitch::new(PitchName::C, 4), Duration::new(DurationType::Quarter, 0), None);

        let phrase = staff.add_phrase();
        phrase.borrow_mut().add_modification(PhraseModificationType::Tuplet { num_beats: 3, into_beats: 2 });
        phrase.borrow_mut().add_note(Pitch::new(PitchName::D, 3), Duration::new(DurationType::Quarter, 0), None);
        let chord = phrase.borrow_mut().add_chord();
        chord.borrow_mut().add_note(Pitch::new(PitchName::E, 4), Duration::new(DurationType::Quarter, 0), None);
        chord.borrow_mut().add_note(Pitch::new(PitchName::D, 4), Duration::new(DurationType::Quarter, 0), None);
        phrase.borrow_mut().add_note(Pitch::new(PitchName::C, 2), Duration::new(DurationType::Quarter, 0), None);

        let _ = staff.add_note(Pitch::new(PitchName::C, 4), Duration::new(DurationType::Quarter, 0), None);

        let phrase = staff.add_phrase();
        phrase.borrow_mut().add_modification(PhraseModificationType::Tuplet { num_beats: 3, into_beats: 2 });
        phrase.borrow_mut().add_note(Pitch::new(PitchName::D, 3), Duration::new(DurationType::Quarter, 0), None);
        phrase.borrow_mut().add_note(Pitch::new(PitchName::E, 4), Duration::new(DurationType::Quarter, 0), None);
        phrase.borrow_mut().add_note(Pitch::new(PitchName::C, 2), Duration::new(DurationType::Quarter, 0), None);

        let phrase = staff.add_phrase();
        phrase.borrow_mut().add_modification(PhraseModificationType::Tuplet { num_beats: 3, into_beats: 2 });
        phrase.borrow_mut().add_note(Pitch::new(PitchName::F, 4), Duration::new(DurationType::Quarter, 0), None);
        phrase.borrow_mut().add_note(Pitch::new(PitchName::D, 2), Duration::new(DurationType::Quarter, 0), None);
        phrase.borrow_mut().add_note(Pitch::new(PitchName::C, 4), Duration::new(DurationType::Quarter, 0), None);

        composition
    };

    assert_eq!(translate(&composition).unwrap(), include_str!("projects/triplets.xml"));
}

#[test]
fn test_repeat() {
    let composition = {
        let mut composition = Composition::new("untitled", None, None, None);
        let part = composition.add_part("part0");
        let section = part.add_section("sec0");
        let mut section = section.borrow_mut();

        {
            let section = section.add_section("sec1");
            let mut section = section.borrow_mut();
            section.add_modification(SectionModificationType::Repeat { num_times: 0 });
            let staff = section.add_staff("staff0", None, None, None);
            let mut staff = staff.borrow_mut();
            staff.add_note(Pitch::new(PitchName::F, 3), Duration::new(DurationType::Quarter, 0), None);
            staff.add_note(Pitch::new(PitchName::A, 1), Duration::new(DurationType::Quarter, 0), None);
        }

        {
            let section = section.add_section("sec2");
            let mut section = section.borrow_mut();
            section.add_modification(SectionModificationType::Repeat { num_times: 1 });
            let staff = section.add_staff("staff0", None, None, None);
            let mut staff = staff.borrow_mut();
            staff.add_note(Pitch::new(PitchName::E, 4), Duration::new(DurationType::Quarter, 0), None);
            staff.add_note(Pitch::new(PitchName::B, 5), Duration::new(DurationType::Quarter, 0), None);
        }

        {
            let section = section.add_section("sec3");
            let mut section = section.borrow_mut();
            section.add_modification(SectionModificationType::Repeat { num_times: 2 });
            let staff = section.add_staff("staff0", None, None, None);
            let mut staff = staff.borrow_mut();
            staff.add_note(Pitch::new(PitchName::D, 2), Duration::new(DurationType::Quarter, 0), None);
            staff.add_note(Pitch::new(PitchName::E, 3), Duration::new(DurationType::Quarter, 0), None);
        }

        composition
    };

    assert_eq!(translate(&composition).unwrap(), include_str!("projects/repeat.xml"));
}
