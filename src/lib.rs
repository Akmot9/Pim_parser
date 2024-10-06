mod errors;
mod validations;

use std::convert::TryFrom;

use errors::PimParseError;
use validations::{validate_length, validate_type, validate_version};

/// Représente un paquet PIM.

#[derive(Debug)]
pub struct PimPacket {
    pub version: u8,
    pub type_: u8,
    // Ajoute d'autres champs nécessaires pour ton paquet PIM
}

impl TryFrom<&[u8]> for PimPacket {
    type Error = PimParseError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        // 1. Valide la longueur du paquet
        validate_length(data)?;

        // 2. Extrait et valide la version
        let version = data[0] >> 4;
        let version = validate_version(version)?;

        // 3. Extrait et valide le type de paquet
        let type_ = data[0] & 0x0F;
        let type_ = validate_type(type_)?;

        // Retourne un PimPacket si tout est correct
        Ok(PimPacket {
            version,
            type_,
            // Ajoute d'autres champs si nécessaire
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_pim_packet() {
        let packet_data: &[u8] = &[0x21, 0x00, 0x00, 0x00]; // Exemple de paquet PIM

        let result = PimPacket::try_from(packet_data);
        assert!(result.is_ok());

        let pim_packet = result.unwrap();
        assert_eq!(pim_packet.version, 2);
        assert_eq!(pim_packet.type_, 1);
    }

    #[test]
    fn test_invalid_pim_packet_length() {
        let invalid_packet_data: &[u8] = &[0x20]; // Paquet trop court

        let result = PimPacket::try_from(invalid_packet_data);
        assert!(matches!(result, Err(PimParseError::PacketTooShort)));
    }

    #[test]
    fn test_invalid_pim_packet_version() {
        let invalid_packet_data: &[u8] = &[0x31, 0x00, 0x00, 0x00]; // Version incorrecte

        let result = PimPacket::try_from(invalid_packet_data);
        assert!(matches!(result, Err(PimParseError::InvalidVersion(3))));
    }

    #[test]
    fn test_custom_pim_packet() {
        // Le paquet de test fourni : 20 00 9f f4 00 01 00 02 00 69
        let packet_data: &[u8] = &[0x20, 0x00, 0x9f, 0xf4, 0x00, 0x01, 0x00, 0x02, 0x00, 0x69];

        let result = PimPacket::try_from(packet_data);
        assert!(result.is_ok());

        let pim_packet = result.unwrap();
        assert_eq!(pim_packet.version, 2); // Version PIM correcte
        assert_eq!(pim_packet.type_, 0); // Type de paquet
    }
}
