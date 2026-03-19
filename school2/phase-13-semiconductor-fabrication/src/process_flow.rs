//! Semiconductor process flow: named steps and simple ordering validation.

#[derive(Debug, Clone, PartialEq)]
pub enum ProcessStep {
    Oxidation,
    Lithography,
    Etch,
    Deposition,
    Implant,
    Anneal,
    MetallizationDeposition,
    ChemicalMechanicalPlanarization,
}

/// A simplified process sequence checker — flags if Anneal does not follow Implant.
pub fn validate_sequence(steps: &[ProcessStep]) -> Result<(), String> {
    for (i, step) in steps.iter().enumerate() {
        if *step == ProcessStep::Implant {
            let next = steps.get(i + 1);
            if next != Some(&ProcessStep::Anneal) {
                return Err(format!("Implant at step {} should be followed by Anneal", i));
            }
        }
    }
    Ok(())
}
