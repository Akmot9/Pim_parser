use crate::errors::PimParseError;

/// Fonction pour valider la taille du paquet.
pub fn validate_length(data: &[u8]) -> Result<(), PimParseError> {
    if data.len() < 4 {
        return Err(PimParseError::PacketTooShort);
    }
    Ok(())
}

/// Fonction pour valider la version du paquet.
pub fn validate_version(version: u8) -> Result<u8, PimParseError> {
    if version != 2 {
        return Err(PimParseError::InvalidVersion(version));
    }
    Ok(version)
}

/// Fonction pour valider le type de paquet.
pub fn validate_type(type_: u8) -> Result<u8, PimParseError> {
    // Ici tu peux ajouter d'autres validations selon les types valides
    Ok(type_)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::PimParseError;

    #[test]
    fn test_validate_length_valid() {
        let data: &[u8] = &[0x21, 0x00, 0x00, 0x00];
        let result = validate_length(data);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_length_invalid() {
        let data: &[u8] = &[0x21];
        let result = validate_length(data);
        assert!(matches!(result, Err(PimParseError::PacketTooShort)));
    }

    #[test]
    fn test_validate_version_valid() {
        let version = 2;
        let result = validate_version(version);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2);
    }

    #[test]
    fn test_validate_version_invalid() {
        let version = 3;
        let result = validate_version(version);
        assert!(matches!(result, Err(PimParseError::InvalidVersion(3))));
    }

    #[test]
    fn test_validate_type_valid() {
        let type_ = 1;
        let result = validate_type(type_);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
    }

    #[test]
    fn test_validate_type_invalid() {
        // Ajoute plus de conditions pour vérifier si un type est invalide
        // Par exemple, si on souhaite n'accepter que certains types, on ajoutera cette logique dans `validate_type`
        let type_ = 10;
        let result = validate_type(type_);
        assert!(result.is_ok()); // Ici on valide tout type, donc on attend OK
    }
}
