use anchor_lang::prelude::*;
use anchor_lang::Discriminator;
use std::io::{Read, Write};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VerificationLevel {
    Partial { num_signatures: u8 },
    Full,
}

impl anchor_lang::prelude::borsh::ser::BorshSerialize for VerificationLevel {
    fn serialize<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        match self {
            VerificationLevel::Partial { num_signatures } => {
                writer.write_all(&[0])?;
                num_signatures.serialize(writer)?;
            }
            VerificationLevel::Full => {
                writer.write_all(&[1])?;
            }
        }
        Ok(())
    }
}

impl anchor_lang::prelude::borsh::de::BorshDeserialize for VerificationLevel {
    fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        if buf.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "Empty buffer",
            ));
        }
        let variant = buf[0];
        *buf = &buf[1..];
        match variant {
            0 => {
                let num_signatures = u8::deserialize(buf)?;
                Ok(VerificationLevel::Partial { num_signatures })
            }
            1 => Ok(VerificationLevel::Full),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid variant",
            )),
        }
    }
    fn deserialize_reader<R: Read>(reader: &mut R) -> std::io::Result<Self> {
        let mut variant_buf = [0u8; 1];
        reader.read_exact(&mut variant_buf)?;
        match variant_buf[0] {
            0 => {
                let mut sigs_buf = [0u8; 1];
                reader.read_exact(&mut sigs_buf)?;
                Ok(VerificationLevel::Partial {
                    num_signatures: sigs_buf[0],
                })
            }
            1 => Ok(VerificationLevel::Full),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid variant",
            )),
        }
    }
}

#[cfg(feature = "idl-build")]
impl anchor_lang::IdlBuild for VerificationLevel {}

#[derive(Clone)]
pub struct PriceUpdateV2 {
    pub write_authority: Pubkey,
    pub verification_level: VerificationLevel,
    pub price_message: PriceFeedMessage,
    pub posted_slot: u64,
}

impl Discriminator for PriceUpdateV2 {
    const DISCRIMINATOR: [u8; 8] = [34, 241, 35, 99, 157, 126, 244, 205];
}

impl Owner for PriceUpdateV2 {
    fn owner() -> Pubkey {
        // Pyth Receiver Program ID: rec5EKMGg6MxZYaMdyBfgwp4d5rB9T1VQH5pJv5LtFJ
        let bytes = [
            12, 183, 250, 187, 82, 247, 166, 72, 187, 91, 49, 125, 154, 1, 139, 144, 87, 203, 2,
            71, 116, 250, 254, 1, 230, 196, 223, 152, 204, 56, 88, 129,
        ];
        Pubkey::new_from_array(bytes)
    }
}

impl AccountSerialize for PriceUpdateV2 {
    fn try_serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer
            .write_all(&PriceUpdateV2::DISCRIMINATOR)
            .map_err(|_| ProgramError::InvalidAccountData)?;
        anchor_lang::prelude::borsh::ser::BorshSerialize::serialize(self, writer)
            .map_err(|_| ProgramError::InvalidAccountData.into())
    }
}

impl AccountDeserialize for PriceUpdateV2 {
    fn try_deserialize(buf: &mut &[u8]) -> Result<Self> {
        if buf.len() < 8 {
            return Err(ProgramError::AccountDataTooSmall.into());
        }
        if buf[..8] != PriceUpdateV2::DISCRIMINATOR {
            return Err(ProgramError::InvalidAccountData.into());
        }
        let mut data = &buf[8..];
        let result = anchor_lang::prelude::borsh::de::BorshDeserialize::deserialize(&mut data)
            .map_err(|_| ProgramError::InvalidAccountData)?;
        Ok(result)
    }

    fn try_deserialize_unchecked(buf: &mut &[u8]) -> Result<Self> {
        anchor_lang::prelude::borsh::de::BorshDeserialize::deserialize(buf)
            .map_err(|_| ProgramError::InvalidAccountData.into())
    }
}

impl anchor_lang::prelude::borsh::ser::BorshSerialize for PriceUpdateV2 {
    fn serialize<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        anchor_lang::prelude::borsh::ser::BorshSerialize::serialize(&self.write_authority, writer)?;
        anchor_lang::prelude::borsh::ser::BorshSerialize::serialize(
            &self.verification_level,
            writer,
        )?;
        anchor_lang::prelude::borsh::ser::BorshSerialize::serialize(&self.price_message, writer)?;
        anchor_lang::prelude::borsh::ser::BorshSerialize::serialize(&self.posted_slot, writer)?;
        Ok(())
    }
}

impl anchor_lang::prelude::borsh::de::BorshDeserialize for PriceUpdateV2 {
    fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        Ok(Self {
            write_authority: Pubkey::deserialize(buf)?,
            verification_level: VerificationLevel::deserialize(buf)?,
            price_message: PriceFeedMessage::deserialize(buf)?,
            posted_slot: u64::deserialize(buf)?,
        })
    }
    fn deserialize_reader<R: Read>(reader: &mut R) -> std::io::Result<Self> {
        Ok(Self {
            write_authority: Pubkey::deserialize_reader(reader)?,
            verification_level: VerificationLevel::deserialize_reader(reader)?,
            price_message: PriceFeedMessage::deserialize_reader(reader)?,
            posted_slot: u64::deserialize_reader(reader)?,
        })
    }
}

#[cfg(feature = "idl-build")]
impl anchor_lang::IdlBuild for PriceUpdateV2 {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PriceFeedMessage {
    pub feed_id: [u8; 32],
    pub price: i64,
    pub conf: u64,
    pub exponent: i32,
    pub publish_time: i64,
    pub prev_publish_time: i64,
    pub ema_price: i64,
    pub ema_conf: u64,
}

impl anchor_lang::prelude::borsh::ser::BorshSerialize for PriceFeedMessage {
    fn serialize<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&self.feed_id)?;
        anchor_lang::prelude::borsh::ser::BorshSerialize::serialize(&self.price, writer)?;
        anchor_lang::prelude::borsh::ser::BorshSerialize::serialize(&self.conf, writer)?;
        anchor_lang::prelude::borsh::ser::BorshSerialize::serialize(&self.exponent, writer)?;
        anchor_lang::prelude::borsh::ser::BorshSerialize::serialize(&self.publish_time, writer)?;
        anchor_lang::prelude::borsh::ser::BorshSerialize::serialize(
            &self.prev_publish_time,
            writer,
        )?;
        anchor_lang::prelude::borsh::ser::BorshSerialize::serialize(&self.ema_price, writer)?;
        anchor_lang::prelude::borsh::ser::BorshSerialize::serialize(&self.ema_conf, writer)?;
        Ok(())
    }
}

impl anchor_lang::prelude::borsh::de::BorshDeserialize for PriceFeedMessage {
    fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut feed_id = [0u8; 32];
        buf.read_exact(&mut feed_id)?;
        Ok(Self {
            feed_id,
            price: i64::deserialize(buf)?,
            conf: u64::deserialize(buf)?,
            exponent: i32::deserialize(buf)?,
            publish_time: i64::deserialize(buf)?,
            prev_publish_time: i64::deserialize(buf)?,
            ema_price: i64::deserialize(buf)?,
            ema_conf: u64::deserialize(buf)?,
        })
    }
    fn deserialize_reader<R: Read>(reader: &mut R) -> std::io::Result<Self> {
        let mut feed_id = [0u8; 32];
        reader.read_exact(&mut feed_id)?;
        Ok(Self {
            feed_id,
            price: i64::deserialize_reader(reader)?,
            conf: u64::deserialize_reader(reader)?,
            exponent: i32::deserialize_reader(reader)?,
            publish_time: i64::deserialize_reader(reader)?,
            prev_publish_time: i64::deserialize_reader(reader)?,
            ema_price: i64::deserialize_reader(reader)?,
            ema_conf: u64::deserialize_reader(reader)?,
        })
    }
}

#[cfg(feature = "idl-build")]
impl anchor_lang::IdlBuild for PriceFeedMessage {}

pub struct Price {
    pub price: i64,
    pub conf: u64,
    pub exponent: i32,
    pub publish_time: i64,
}

impl PriceUpdateV2 {
    pub fn get_price_no_older_than(
        &self,
        _clock: &Clock,
        _maximum_age: u64,
        _feed_id: &[u8; 32],
    ) -> Result<Price> {
        Ok(Price {
            price: self.price_message.price,
            conf: self.price_message.conf,
            exponent: self.price_message.exponent,
            publish_time: self.price_message.publish_time,
        })
    }
}

pub fn get_feed_id_from_hex(input: &str) -> Result<[u8; 32]> {
    let mut feed_id = [0u8; 32];
    let hex_str = if input.starts_with("0x") {
        &input[2..]
    } else {
        input
    };

    for i in 0..32 {
        feed_id[i] = u8::from_str_radix(&hex_str[2 * i..2 * i + 2], 16)
            .map_err(|_| ProgramError::InvalidArgument)?;
    }

    Ok(feed_id)
}
