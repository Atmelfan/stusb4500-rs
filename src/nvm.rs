use hal::i2c::I2c;

use crate::{Error, NvmCtrl0, NvmCtrl1, NvmCtrl1Opcode, Register, STUSB4500};

pub const DEFAULT_NVM_DATA: [[u8; 8]; 5] = [
    [0x00, 0x00, 0xB0, 0xAB, 0x00, 0x45, 0x00, 0x00],
    [0x10, 0x40, 0x9C, 0x1C, 0xFF, 0x01, 0x3C, 0xDF],
    [0x02, 0x40, 0x0F, 0x00, 0x32, 0x00, 0xFC, 0xF1],
    [0x00, 0x19, 0x56, 0xAF, 0xF5, 0x35, 0x5F, 0x00],
    [0x00, 0x4B, 0x90, 0x21, 0x43, 0x00, 0x40, 0xFB],
];

pub struct STUSB4500Nvm<'a, I2C> {
    inner: &'a mut STUSB4500<I2C>,
}

impl<I2C, E> STUSB4500Nvm<'_, I2C>
where
    I2C: I2c<Error = E>,
{
    const DEFAULT_PASSWORD: u8 = 0x47;

    pub(crate) fn unlock(inner: &mut STUSB4500<I2C>) -> Result<STUSB4500Nvm<I2C>, Error<E>> {
        inner.write(Register::NvmPassword, STUSB4500Nvm::<I2C>::DEFAULT_PASSWORD)?;
        inner.write(Register::NvmCtrl0, 0x00)?;
        inner.write(
            Register::NvmCtrl0,
            (NvmCtrl0::Power | NvmCtrl0::Enable).bits(),
        )?;

        Ok(STUSB4500Nvm { inner })
    }

    /// Lock the NVM
    pub fn lock(self) -> Result<(), Error<E>> {
        self.inner
            .write(Register::NvmCtrl0, NvmCtrl0::Enable.bits())?;
        self.inner.write(Register::NvmCtrl1, 0x00)?;
        self.inner.write(Register::NvmPassword, 0x00)
    }

    /// Read the NVM data (all five sectors)
    ///
    /// The NVM data is used to set the configuration on power-up. It can be decoded by the [GUI
    /// application][gui].
    ///
    /// [gui]: https://www.st.com/en/embedded-software/stsw-stusb002.html
    pub fn read_sectors(&mut self) -> Result<[[u8; 8]; 5], Error<E>> {
        let mut buf = [[0x00; 8]; 5];
        for (i, sector) in buf.iter_mut().enumerate() {
            *sector = self.read_sector(i as u8)?;
        }
        Ok(buf)
    }

    /// Write the NVM data (all five sectors)
    ///
    /// The NVM data is used to set the configuration on power-up. It can be generated by the [GUI
    /// application][gui].
    ///
    /// [gui]: https://www.st.com/en/embedded-software/stsw-stusb002.html
    pub fn write_sectors(&mut self, sectors: [[u8; 8]; 5]) -> Result<(), Error<E>> {
        self.erase_sectors()?;
        for (i, sector) in sectors.iter().enumerate() {
            self.write_sector(i as u8, sector)?;
        }
        Ok(())
    }

    fn issue_request(&mut self) -> Result<(), Error<E>> {
        self.issue_request_with_sector(0)
    }

    fn issue_request_with_sector(&mut self, sector: u8) -> Result<(), Error<E>> {
        self.inner.write(
            Register::NvmCtrl0,
            sector | (NvmCtrl0::Power | NvmCtrl0::Enable | NvmCtrl0::Request).bits(),
        )?;

        while NvmCtrl0::from_bits_truncate(self.inner.read(Register::NvmCtrl0)?)
            .contains(NvmCtrl0::Request)
        {}

        Ok(())
    }

    fn read_sector(&mut self, sector: u8) -> Result<[u8; 8], Error<E>> {
        self.inner
            .write(Register::NvmCtrl1, NvmCtrl1Opcode::ReadSector as u8)?;
        self.issue_request_with_sector(sector)?;

        let mut buf = [0x00; 8];
        self.inner
            .i2c
            .write(self.inner.address, &[Register::RWBuffer as u8])
            .map_err(|err| Error::I2CError(err))?;
        self.inner
            .i2c
            .read(self.inner.address, &mut buf)
            .map_err(|err| Error::I2CError(err))?;
        Ok(buf)
    }

    fn write_sector(&mut self, sector: u8, data: &[u8; 8]) -> Result<(), Error<E>> {
        let mut buf = [0x00; 9];
        buf[0] = Register::RWBuffer as u8;
        buf[1..].copy_from_slice(data);

        self.inner
            .i2c
            .write(self.inner.address, &buf)
            .map_err(|err| Error::I2CError(err))?;
        self.inner
            .write(Register::NvmCtrl1, NvmCtrl1Opcode::LoadPlr as u8)?;
        self.issue_request()?;

        self.inner
            .write(Register::NvmCtrl1, NvmCtrl1Opcode::WriteSector as u8)?;
        self.issue_request_with_sector(sector)
    }

    fn erase_sectors(&mut self) -> Result<(), Error<E>> {
        self.inner.write(
            Register::NvmCtrl1,
            NvmCtrl1Opcode::LoadSer as u8
                | (NvmCtrl1::EraseSector0
                    | NvmCtrl1::EraseSector1
                    | NvmCtrl1::EraseSector2
                    | NvmCtrl1::EraseSector3
                    | NvmCtrl1::EraseSector4)
                    .bits(),
        )?;
        self.issue_request()?;

        self.inner
            .write(Register::NvmCtrl1, NvmCtrl1Opcode::EraseSectors as u8)?;
        self.issue_request()
    }
}