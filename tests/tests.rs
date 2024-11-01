use amm::Composition;
use amm::note::{DurationType, Duration, Accidental, Pitch, PitchName};
use amm::context::{Dynamic, Key, Tempo, KeySignature, KeyMode, TimeSignature, TimeSignatureType, TempoSuggestion, TempoMarking};
use amm::modification::{PhraseModificationType, NoteModificationType, SectionModificationType, DirectionType};

use amm_sdk_netsblox::*;

#[test]
fn test_notes() {
    let composition = {
        let mut composition = Composition::new("untitled", None, None, None);
        composition.set_tempo(Tempo::new(Duration::new(DurationType::Quarter, 0), 87));
        composition.set_starting_key(Key::new(KeySignature::DFlat, KeyMode::Major));
        composition.set_starting_time_signature(TimeSignature::new(TimeSignatureType::CutTime));
        let part = composition.add_part("part0");
        let section = part.add_section("sec0");
        let staff = section.add_staff("staff0");

        staff.add_note(Pitch::new(PitchName::G, 2), Duration::new(DurationType::Maxima, 0), None);
        staff.add_note(Pitch::new(PitchName::B, 3), Duration::new(DurationType::Long, 0), None);
        staff.add_note(Pitch::new(PitchName::A, 4), Duration::new(DurationType::Breve, 0), None);
        staff.add_note(Pitch::new(PitchName::D, 1), Duration::new(DurationType::Whole, 0), None);
        staff.add_note(Pitch::new(PitchName::E, 2), Duration::new(DurationType::Half, 0), None);
        staff.add_note(Pitch::new(PitchName::A, 3), Duration::new(DurationType::Quarter, 0), None);
        staff.add_note(Pitch::new(PitchName::D, 4), Duration::new(DurationType::Eighth, 0), None);
        staff.add_note(Pitch::new(PitchName::B, 3), Duration::new(DurationType::Sixteenth, 0), None);
        staff.add_note(Pitch::new(PitchName::E, 2), Duration::new(DurationType::ThirtySecond, 0), None);
        staff.add_note(Pitch::new(PitchName::E, 1), Duration::new(DurationType::SixtyFourth, 0), None);

        staff.add_note(Pitch::new(PitchName::G, 2), Duration::new(DurationType::Maxima, 1), None);
        staff.add_note(Pitch::new(PitchName::B, 3), Duration::new(DurationType::Long, 1), None);
        staff.add_note(Pitch::new(PitchName::A, 4), Duration::new(DurationType::Breve, 1), None);
        staff.add_note(Pitch::new(PitchName::D, 1), Duration::new(DurationType::Whole, 1), None);
        staff.add_note(Pitch::new(PitchName::E, 2), Duration::new(DurationType::Half, 1), None);
        staff.add_note(Pitch::new(PitchName::A, 3), Duration::new(DurationType::Quarter, 1), None);
        staff.add_note(Pitch::new(PitchName::D, 4), Duration::new(DurationType::Eighth, 1), None);
        staff.add_note(Pitch::new(PitchName::B, 3), Duration::new(DurationType::Sixteenth, 1), None);
        staff.add_note(Pitch::new(PitchName::E, 2), Duration::new(DurationType::ThirtySecond, 1), None);
        staff.add_note(Pitch::new(PitchName::E, 1), Duration::new(DurationType::SixtyFourth, 1), None);

        staff.add_note(Pitch::new(PitchName::G, 2), Duration::new(DurationType::Maxima, 2), None);
        staff.add_note(Pitch::new(PitchName::B, 3), Duration::new(DurationType::Long, 2), None);
        staff.add_note(Pitch::new(PitchName::A, 4), Duration::new(DurationType::Breve, 2), None);
        staff.add_note(Pitch::new(PitchName::D, 1), Duration::new(DurationType::Whole, 2), None);
        staff.add_note(Pitch::new(PitchName::E, 2), Duration::new(DurationType::Half, 2), None);
        staff.add_note(Pitch::new(PitchName::A, 3), Duration::new(DurationType::Quarter, 2), None);
        staff.add_note(Pitch::new(PitchName::D, 4), Duration::new(DurationType::Eighth, 2), None);
        staff.add_note(Pitch::new(PitchName::B, 3), Duration::new(DurationType::Sixteenth, 2), None);
        staff.add_note(Pitch::new(PitchName::E, 2), Duration::new(DurationType::ThirtySecond, 2), None);
        staff.add_note(Pitch::new(PitchName::E, 1), Duration::new(DurationType::SixtyFourth, 2), None);

        composition
    };

    let trans = translate(&composition).unwrap();
    if trans != include_str!("projects/notes.xml") {
        panic!("{trans}");
    }
}

#[test]
fn test_chords() {
    let composition = {
        let mut composition = Composition::new("untitled", None, None, None);
        composition.set_copyright("Original Music Do Not Steal (2024)");
        let part = composition.add_part("small harping thingy");
        let section = part.add_section("sec0");
        let staff = section.add_staff("staff0");

        let _ = staff.add_chord();

        let chord = staff.add_chord();
        chord.add_note(Pitch::new(PitchName::B, 2), Duration::new(DurationType::Eighth, 2), None);

        let chord = staff.add_chord();
        chord.add_note(Pitch::new(PitchName::B, 2), Duration::new(DurationType::Half, 1), None);
        chord.add_note(Pitch::new(PitchName::E, 4), Duration::new(DurationType::Half, 1), None);

        let chord = staff.add_chord();
        chord.add_note(Pitch::new(PitchName::D, 4), Duration::new(DurationType::Quarter, 0), None);
        chord.add_note(Pitch::new(PitchName::E, 2), Duration::new(DurationType::Quarter, 0), None);
        chord.add_note(Pitch::new(PitchName::F, 3), Duration::new(DurationType::Quarter, 0), None);

        let chord = staff.add_chord();
        chord.add_note(Pitch::new(PitchName::D, 4), Duration::new(DurationType::Quarter, 0), None);
        chord.add_note(Pitch::new(PitchName::E, 2), Duration::new(DurationType::Eighth, 0), None);
        chord.add_note(Pitch::new(PitchName::F, 3), Duration::new(DurationType::Sixteenth, 1), None);

        let chord = staff.add_chord();
        chord.add_note(Pitch::new(PitchName::B, 2), Duration::new(DurationType::Half, 1), None);
        chord.add_note(Pitch::new_rest(), Duration::new(DurationType::Half, 1), None);

        let chord = staff.add_chord();
        chord.add_note(Pitch::new(PitchName::B, 3), Duration::new(DurationType::Half, 1), None);
        chord.add_note(Pitch::new_rest(), Duration::new(DurationType::Whole, 0), None);

        let chord = staff.add_chord();
        chord.add_note(Pitch::new(PitchName::A, 2), Duration::new(DurationType::Half, 1), None);
        chord.add_note(Pitch::new_rest(), Duration::new(DurationType::Quarter, 2), None);

        let chord = staff.add_chord();
        chord.add_note(Pitch::new_rest(), Duration::new(DurationType::Quarter, 0), None);
        chord.add_note(Pitch::new(PitchName::G, 3), Duration::new(DurationType::Half, 0), None);

        let chord = staff.add_chord();
        chord.add_note(Pitch::new_rest(), Duration::new(DurationType::Half, 1), None);
        chord.add_note(Pitch::new(PitchName::F, 5), Duration::new(DurationType::Half, 0), None);

        let chord = staff.add_chord();
        chord.add_note(Pitch::new_rest(), Duration::new(DurationType::Half, 2), None);

        let chord = staff.add_chord();
        chord.add_note(Pitch::new_rest(), Duration::new(DurationType::Half, 0), None);
        chord.add_note(Pitch::new_rest(), Duration::new(DurationType::Quarter, 1), None);
        chord.add_note(Pitch::new_rest(), Duration::new(DurationType::Half, 1), None);
        chord.add_note(Pitch::new_rest(), Duration::new(DurationType::Quarter, 2), None);

        composition
    };

    let trans = translate(&composition).unwrap();
    if trans != include_str!("projects/chords.xml") {
        panic!("{trans}");
    }
}

#[test]
fn test_note_mods() {
    let composition = {
        let mut composition = Composition::new("untitled", None, None, None);
        composition.set_publisher("Disco Punk 2077");
        let part = composition.add_part("Electronical Guitars");
        let section = part.add_section("sec0");
        let staff = section.add_staff("staff0");

        let _ = staff.add_note(Pitch::new(PitchName::C, 4), Duration::new(DurationType::Quarter, 0), None);

        let note = staff.add_note(Pitch::new(PitchName::E, 4), Duration::new(DurationType::Quarter, 0), None);
        note.add_modification(NoteModificationType::Dynamic { dynamic: Dynamic::Piano(0) });

        let _ = staff.add_note(Pitch::new(PitchName::C, 3), Duration::new(DurationType::Quarter, 0), None);

        let note = staff.add_note(Pitch::new(PitchName::D, 3), Duration::new(DurationType::Quarter, 0), None);
        note.add_modification(NoteModificationType::Dynamic { dynamic: Dynamic::Forte(0) });

        let note = staff.add_note(Pitch::new(PitchName::E, 3), Duration::new(DurationType::Quarter, 0), None);
        note.add_modification(NoteModificationType::Dynamic { dynamic: Dynamic::Forte(0) });

        let _ = staff.add_note(Pitch::new(PitchName::C, 3), Duration::new(DurationType::Quarter, 0), None);

        let note = staff.add_note(Pitch::new(PitchName::E, 4), Duration::new(DurationType::Quarter, 0), None);
        note.add_modification(NoteModificationType::Turn { upper: true, delayed: false, vertical: false });

        let note = staff.add_note(Pitch::new(PitchName::E, 4), Duration::new(DurationType::Quarter, 0), None);
        note.add_modification(NoteModificationType::Turn { upper: false, delayed: false, vertical: false });

        let _ = staff.add_note(Pitch::new(PitchName::C, 3), Duration::new(DurationType::Quarter, 0), None);

        let note = staff.add_note(Pitch::new(PitchName::D, 3), Duration::new(DurationType::Quarter, 0), None);
        note.add_modification(NoteModificationType::Dynamic { dynamic: Dynamic::Piano(0) });
        note.add_modification(NoteModificationType::Accent);

        let note = staff.add_note(Pitch::new(PitchName::E, 3), Duration::new(DurationType::Quarter, 0), None);
        note.add_modification(NoteModificationType::Accent);
        note.add_modification(NoteModificationType::Dynamic { dynamic: Dynamic::Piano(0) });

        let note = staff.add_note(Pitch::new(PitchName::D, 3), Duration::new(DurationType::Quarter, 0), None);
        note.add_modification(NoteModificationType::Accent);
        note.add_modification(NoteModificationType::Dynamic { dynamic: Dynamic::Piano(0) });
        note.add_modification(NoteModificationType::Staccato);

        let note = staff.add_note(Pitch::new(PitchName::E, 3), Duration::new(DurationType::Quarter, 0), None);
        note.add_modification(NoteModificationType::Staccato);
        note.add_modification(NoteModificationType::Accent);
        note.add_modification(NoteModificationType::Dynamic { dynamic: Dynamic::Piano(0) });

        let note = staff.add_note(Pitch::new(PitchName::C, 2), Duration::new(DurationType::Quarter, 0), None);
        note.add_modification(NoteModificationType::Dynamic { dynamic: Dynamic::Piano(0) });
        note.add_modification(NoteModificationType::Accent);

        let note = staff.add_note(Pitch::new(PitchName::G, 4), Duration::new(DurationType::Quarter, 0), None);
        note.add_modification(NoteModificationType::Accent);
        note.add_modification(NoteModificationType::Dynamic { dynamic: Dynamic::Piano(0) });

        composition
    };

    let trans = translate(&composition).unwrap();
    if trans != include_str!("projects/note-mods.xml") {
        panic!("{trans}");
    }
}

#[test]
fn test_triplets() {
    let composition = {
        let mut composition = Composition::new("untitled", None, None, None);
        composition.add_arranger("Glob Simpson");
        let part = composition.add_part("part0");
        let section = part.add_section("sec0");
        let staff = section.add_staff("staff0");

        let _ = staff.add_note(Pitch::new(PitchName::C, 4), Duration::new(DurationType::Quarter, 0), None);

        let phrase = staff.add_phrase();
        phrase.add_modification(PhraseModificationType::Tuplet { num_beats: 3, into_beats: 2 });
        phrase.add_note(Pitch::new(PitchName::D, 3), Duration::new(DurationType::Quarter, 0), None);
        let chord = phrase.add_chord();
        chord.add_note(Pitch::new(PitchName::E, 4), Duration::new(DurationType::Quarter, 0), None);
        chord.add_note(Pitch::new(PitchName::D, 4), Duration::new(DurationType::Quarter, 0), None);
        phrase.add_note(Pitch::new(PitchName::C, 2), Duration::new(DurationType::Quarter, 0), None);

        let _ = staff.add_note(Pitch::new(PitchName::C, 4), Duration::new(DurationType::Quarter, 0), None);

        let phrase = staff.add_phrase();
        phrase.add_modification(PhraseModificationType::Tuplet { num_beats: 3, into_beats: 2 });
        phrase.add_note(Pitch::new(PitchName::D, 3), Duration::new(DurationType::Quarter, 0), None);
        phrase.add_note(Pitch::new(PitchName::E, 4), Duration::new(DurationType::Quarter, 0), None);
        phrase.add_note(Pitch::new(PitchName::C, 2), Duration::new(DurationType::Quarter, 0), None);

        let phrase = staff.add_phrase();
        phrase.add_modification(PhraseModificationType::Tuplet { num_beats: 3, into_beats: 2 });
        phrase.add_note(Pitch::new(PitchName::F, 4), Duration::new(DurationType::Quarter, 0), None);
        phrase.add_note(Pitch::new(PitchName::D, 2), Duration::new(DurationType::Quarter, 0), None);
        phrase.add_note(Pitch::new(PitchName::C, 4), Duration::new(DurationType::Quarter, 0), None);

        composition
    };

    let trans = translate(&composition).unwrap();
    if trans != include_str!("projects/triplets.xml") {
        panic!("{trans}");
    }
}

#[test]
fn test_repeat() {
    let composition = {
        let mut composition = Composition::new("untitled", None, None, None);
        composition.add_lyricist("MC Unit Test");
        composition.add_lyricist("Debbie Debs");
        let part = composition.add_part("part0");
        let section = part.add_section("sec0");

        {
            let section = section.add_section("sec1");
            section.add_modification(SectionModificationType::Repeat { num_times: 0 });
            let staff = section.add_staff("staff0");
            staff.add_note(Pitch::new(PitchName::F, 3), Duration::new(DurationType::Quarter, 0), None);
            staff.add_note(Pitch::new(PitchName::A, 1), Duration::new(DurationType::Quarter, 0), None);
        }

        {
            let section = section.add_section("sec2");
            section.add_modification(SectionModificationType::Repeat { num_times: 1 });
            let staff = section.add_staff("staff0");
            staff.add_note(Pitch::new(PitchName::E, 4), Duration::new(DurationType::Quarter, 0), None);
            staff.add_note(Pitch::new(PitchName::B, 5), Duration::new(DurationType::Quarter, 0), None);
        }

        {
            let section = section.add_section("sec3");
            section.add_modification(SectionModificationType::Repeat { num_times: 2 });
            let staff = section.add_staff("staff0");
            staff.add_note(Pitch::new(PitchName::D, 2), Duration::new(DurationType::Quarter, 0), None);
            staff.add_note(Pitch::new(PitchName::E, 3), Duration::new(DurationType::Quarter, 0), None);
        }

        composition
    };

    let trans = translate(&composition).unwrap();
    if trans != include_str!("projects/repeat.xml") {
        panic!("{trans}");
    }
}

#[test]
fn test_rests() {
    let composition = {
        let mut composition = Composition::new("some title", None, None, None);
        composition.add_composer("DJ Devin");
        let part = composition.add_part("some pipes or something");
        let section = part.add_section("sec0");
        let staff = section.add_staff("staff0");

        staff.add_note(Pitch::new_rest(), Duration::new(DurationType::Maxima, 0), None);
        staff.add_note(Pitch::new_rest(), Duration::new(DurationType::Long, 0), None);
        staff.add_note(Pitch::new_rest(), Duration::new(DurationType::Breve, 0), None);
        staff.add_note(Pitch::new_rest(), Duration::new(DurationType::Whole, 0), None);
        staff.add_note(Pitch::new_rest(), Duration::new(DurationType::Half, 0), None);
        staff.add_note(Pitch::new_rest(), Duration::new(DurationType::Quarter, 0), None);
        staff.add_note(Pitch::new_rest(), Duration::new(DurationType::Eighth, 0), None);
        staff.add_note(Pitch::new_rest(), Duration::new(DurationType::Sixteenth, 0), None);
        staff.add_note(Pitch::new_rest(), Duration::new(DurationType::ThirtySecond, 0), None);
        staff.add_note(Pitch::new_rest(), Duration::new(DurationType::SixtyFourth, 0), None);

        staff.add_note(Pitch::new_rest(), Duration::new(DurationType::Maxima, 1), None);
        staff.add_note(Pitch::new_rest(), Duration::new(DurationType::Long, 1), None);
        staff.add_note(Pitch::new_rest(), Duration::new(DurationType::Breve, 1), None);
        staff.add_note(Pitch::new_rest(), Duration::new(DurationType::Whole, 1), None);
        staff.add_note(Pitch::new_rest(), Duration::new(DurationType::Half, 1), None);
        staff.add_note(Pitch::new_rest(), Duration::new(DurationType::Quarter, 1), None);
        staff.add_note(Pitch::new_rest(), Duration::new(DurationType::Eighth, 1), None);
        staff.add_note(Pitch::new_rest(), Duration::new(DurationType::Sixteenth, 1), None);
        staff.add_note(Pitch::new_rest(), Duration::new(DurationType::ThirtySecond, 1), None);
        staff.add_note(Pitch::new_rest(), Duration::new(DurationType::SixtyFourth, 1), None);

        staff.add_note(Pitch::new_rest(), Duration::new(DurationType::Maxima, 2), None);
        staff.add_note(Pitch::new_rest(), Duration::new(DurationType::Long, 2), None);
        staff.add_note(Pitch::new_rest(), Duration::new(DurationType::Breve, 2), None);
        staff.add_note(Pitch::new_rest(), Duration::new(DurationType::Whole, 2), None);
        staff.add_note(Pitch::new_rest(), Duration::new(DurationType::Half, 2), None);
        staff.add_note(Pitch::new_rest(), Duration::new(DurationType::Quarter, 2), None);
        staff.add_note(Pitch::new_rest(), Duration::new(DurationType::Eighth, 2), None);
        staff.add_note(Pitch::new_rest(), Duration::new(DurationType::Sixteenth, 2), None);
        staff.add_note(Pitch::new_rest(), Duration::new(DurationType::ThirtySecond, 2), None);
        staff.add_note(Pitch::new_rest(), Duration::new(DurationType::SixtyFourth, 2), None);

        staff.add_note(Pitch::new_rest(), Duration::new(DurationType::Quarter, 0), None);

        let chord = staff.add_chord();
        chord.add_note(Pitch::new_rest(), Duration::new(DurationType::Eighth, 0), None).add_modification(NoteModificationType::Dynamic { dynamic: Dynamic::Forte(0) });

        staff.add_note(Pitch::new_rest(), Duration::new(DurationType::Quarter, 0), None);

        let chord = staff.add_chord();
        chord.add_note(Pitch::new_rest(), Duration::new(DurationType::Eighth, 0), None).add_modification(NoteModificationType::Dynamic { dynamic: Dynamic::Forte(0) });
        chord.add_note(Pitch::new_rest(), Duration::new(DurationType::Quarter, 0), None).add_modification(NoteModificationType::Dynamic { dynamic: Dynamic::Forte(0) });

        staff.add_note(Pitch::new_rest(), Duration::new(DurationType::Quarter, 0), None);

        let chord = staff.add_chord();
        chord.add_note(Pitch::new_rest(), Duration::new(DurationType::Eighth, 0), None);
        chord.add_note(Pitch::new(PitchName::B, 3), Duration::new(DurationType::Eighth, 0), None).add_modification(NoteModificationType::Dynamic { dynamic: Dynamic::Forte(0) });

        staff.add_note(Pitch::new_rest(), Duration::new(DurationType::Quarter, 0), None);

        let chord = staff.add_chord();
        chord.add_note(Pitch::new(PitchName::A, 2), Duration::new(DurationType::Eighth, 0), None);
        chord.add_note(Pitch::new_rest(), Duration::new(DurationType::Eighth, 0), None);
        chord.add_note(Pitch::new(PitchName::E, 4), Duration::new(DurationType::Eighth, 0), None).add_modification(NoteModificationType::Dynamic { dynamic: Dynamic::Forte(0) });

        staff.add_note(Pitch::new_rest(), Duration::new(DurationType::Quarter, 0), None);

        let chord = staff.add_chord();
        chord.add_note(Pitch::new(PitchName::B, 4), Duration::new(DurationType::Sixteenth, 0), None);
        chord.add_note(Pitch::new_rest(), Duration::new(DurationType::Sixteenth, 0), None).add_modification(NoteModificationType::Dynamic { dynamic: Dynamic::Forte(0) });
        chord.add_note(Pitch::new(PitchName::G, 3), Duration::new(DurationType::Sixteenth, 0), None);

        let chord = staff.add_chord();
        chord.add_note(Pitch::new(PitchName::B, 4), Duration::new(DurationType::Sixteenth, 0), None);
        chord.add_note(Pitch::new_rest(), Duration::new(DurationType::ThirtySecond, 0), None).add_modification(NoteModificationType::Dynamic { dynamic: Dynamic::Forte(0) });
        chord.add_note(Pitch::new(PitchName::G, 3), Duration::new(DurationType::Sixteenth, 0), None);

        composition
    };

    let trans = translate(&composition).unwrap();
    if trans != include_str!("projects/rests.xml") {
        panic!("{trans}");
    }
}

#[test]
fn test_accidentals() {
    let composition = {
        let mut composition = Composition::new("untitled", None, None, None);
        let part = composition.add_part("part0");
        let section = part.add_section("sec0");
        let staff = section.add_staff("staff0");

        staff.add_note(Pitch::new(PitchName::B, 3), Duration::new(DurationType::Quarter, 0), None);
        staff.add_note(Pitch::new(PitchName::B, 3), Duration::new(DurationType::Quarter, 0), Some(Accidental::None));
        staff.add_note(Pitch::new(PitchName::B, 3), Duration::new(DurationType::Quarter, 0), Some(Accidental::Natural));
        staff.add_note(Pitch::new(PitchName::B, 3), Duration::new(DurationType::Quarter, 0), Some(Accidental::Sharp));
        staff.add_note(Pitch::new(PitchName::B, 3), Duration::new(DurationType::Quarter, 0), Some(Accidental::DoubleSharp));
        staff.add_note(Pitch::new(PitchName::B, 3), Duration::new(DurationType::Quarter, 0), Some(Accidental::Flat));
        staff.add_note(Pitch::new(PitchName::B, 3), Duration::new(DurationType::Quarter, 0), Some(Accidental::DoubleFlat));

        staff.add_direction(DirectionType::KeyChange { key: Key::new(KeySignature::EFlat, KeyMode::Major) });

        staff.add_note(Pitch::new(PitchName::E, 4), Duration::new(DurationType::Quarter, 0), None);
        staff.add_note(Pitch::new(PitchName::E, 4), Duration::new(DurationType::Quarter, 0), Some(Accidental::None));
        staff.add_note(Pitch::new(PitchName::E, 4), Duration::new(DurationType::Quarter, 0), Some(Accidental::Natural));
        staff.add_note(Pitch::new(PitchName::E, 4), Duration::new(DurationType::Quarter, 0), Some(Accidental::Sharp));
        staff.add_note(Pitch::new(PitchName::E, 4), Duration::new(DurationType::Quarter, 0), Some(Accidental::DoubleSharp));
        staff.add_note(Pitch::new(PitchName::E, 4), Duration::new(DurationType::Quarter, 0), Some(Accidental::Flat));
        staff.add_note(Pitch::new(PitchName::E, 4), Duration::new(DurationType::Quarter, 0), Some(Accidental::DoubleFlat));

        staff.add_direction(DirectionType::KeyChange { key: Key::new(KeySignature::GSharp, KeyMode::Minor) });

        let chord = staff.add_chord();
        chord.add_note(Pitch::new(PitchName::G, 2), Duration::new(DurationType::Quarter, 0), None);
        chord.add_note(Pitch::new(PitchName::G, 2), Duration::new(DurationType::Quarter, 0), Some(Accidental::Natural));
        chord.add_note(Pitch::new(PitchName::G, 2), Duration::new(DurationType::Quarter, 0), Some(Accidental::Sharp));
        chord.add_note(Pitch::new(PitchName::G, 2), Duration::new(DurationType::Quarter, 0), Some(Accidental::DoubleSharp));
        chord.add_note(Pitch::new(PitchName::G, 2), Duration::new(DurationType::Quarter, 0), Some(Accidental::Flat));
        chord.add_note(Pitch::new(PitchName::G, 2), Duration::new(DurationType::Quarter, 0), Some(Accidental::DoubleFlat));

        composition
    };

    let trans = translate(&composition).unwrap();
    if trans != include_str!("projects/accidentals.xml") {
        panic!("{trans}");
    }
}

#[test]
fn test_tempo() {
    let composition = {
        let mut composition = Composition::new("tempo", None, None, None);
        composition.set_tempo(Tempo { base_note: Duration::new(DurationType::Eighth, 0), beats_per_minute: 54 });
        composition.add_lyricist("MC Unit Test");
        composition.add_lyricist("Debbie Debs");
        let part = composition.add_part("part0");
        let section = part.add_section("sec0");

        {
            let section = section.add_section("sec1");
            section.add_modification(SectionModificationType::TempoExplicit { tempo: Tempo::new(Duration::new(DurationType::Quarter, 0), 32) });
            let staff = section.add_staff("staff0");
            staff.add_note(Pitch::new(PitchName::F, 3), Duration::new(DurationType::Quarter, 0), None);
            staff.add_note(Pitch::new(PitchName::A, 1), Duration::new(DurationType::Quarter, 0), None);
        }

        {
            let section = section.add_section("sec2");
            section.add_modification(SectionModificationType::TempoExplicit { tempo: Tempo::new(Duration::new(DurationType::Half, 0), 40) });
            let staff = section.add_staff("staff0");
            staff.add_note(Pitch::new(PitchName::E, 4), Duration::new(DurationType::Quarter, 0), None);
            staff.add_note(Pitch::new(PitchName::B, 5), Duration::new(DurationType::Quarter, 0), None);
        }

        {
            let section = section.add_section("sec3");
            section.add_modification(SectionModificationType::TempoExplicit { tempo: Tempo::new(Duration::new(DurationType::Sixteenth, 0), 123) });
            let staff = section.add_staff("staff0");
            staff.add_note(Pitch::new(PitchName::D, 2), Duration::new(DurationType::Quarter, 0), None);
            staff.add_note(Pitch::new(PitchName::E, 3), Duration::new(DurationType::Quarter, 0), None);
        }

        {
            let section = section.add_section("sec3");
            section.add_modification(SectionModificationType::TempoImplicit { tempo: TempoSuggestion::new(TempoMarking::Andante) });
            let staff = section.add_staff("staff0");
            staff.add_note(Pitch::new(PitchName::D, 2), Duration::new(DurationType::Quarter, 0), None);
            staff.add_note(Pitch::new(PitchName::E, 3), Duration::new(DurationType::Quarter, 0), None);
        }

        {
            let section = section.add_section("sec3");
            section.add_modification(SectionModificationType::TempoImplicit { tempo: TempoSuggestion::new(TempoMarking::Prestissimo) });
            let staff = section.add_staff("staff0");
            staff.add_note(Pitch::new(PitchName::D, 2), Duration::new(DurationType::Quarter, 0), None);
            staff.add_note(Pitch::new(PitchName::E, 3), Duration::new(DurationType::Quarter, 0), None);
        }

        composition
    };

    let trans = translate(&composition).unwrap();
    if trans != include_str!("projects/tempo.xml") {
        panic!("{trans}");
    }
}
