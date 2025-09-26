#[doc = "Register `SADDR0` reader"]
pub type R = crate::R<SADDR0_SPEC>;
#[doc = "Register `SADDR0` writer"]
pub type W = crate::W<SADDR0_SPEC>;
#[doc = "Field `ADDRESS0` reader - Bit 0 of a 10-bit address"]
pub type ADDRESS0_R = crate::BitReader;
#[doc = "Field `ADDRESS0` writer - Bit 0 of a 10-bit address"]
pub type ADDRESS0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESS7_1` reader - 7-bit address or bits 7:1 of a 10-bit address"]
pub type ADDRESS7_1_R = crate::FieldReader;
#[doc = "Field `ADDRESS7_1` writer - 7-bit address or bits 7:1 of a 10-bit address"]
pub type ADDRESS7_1_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ADDRESS9_8` reader - Highest two bits of a 10-bit address"]
pub type ADDRESS9_8_R = crate::FieldReader;
#[doc = "Field `ADDRESS9_8` writer - Highest two bits of a 10-bit address"]
pub type ADDRESS9_8_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADDFORMAT` reader - Address mode for the I2C slave"]
pub type ADDFORMAT_R = crate::BitReader;
#[doc = "Field `ADDFORMAT` writer - Address mode for the I2C slave"]
pub type ADDFORMAT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Bit 0 of a 10-bit address"]
    #[inline(always)]
    pub fn address0(&self) -> ADDRESS0_R {
        ADDRESS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7-bit address or bits 7:1 of a 10-bit address"]
    #[inline(always)]
    pub fn address7_1(&self) -> ADDRESS7_1_R {
        ADDRESS7_1_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:9 - Highest two bits of a 10-bit address"]
    #[inline(always)]
    pub fn address9_8(&self) -> ADDRESS9_8_R {
        ADDRESS9_8_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 15 - Address mode for the I2C slave"]
    #[inline(always)]
    pub fn addformat(&self) -> ADDFORMAT_R {
        ADDFORMAT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bit 0 of a 10-bit address"]
    #[inline(always)]
    #[must_use]
    pub fn address0(&mut self) -> ADDRESS0_W<SADDR0_SPEC> {
        ADDRESS0_W::new(self, 0)
    }
    #[doc = "Bits 1:7 - 7-bit address or bits 7:1 of a 10-bit address"]
    #[inline(always)]
    #[must_use]
    pub fn address7_1(&mut self) -> ADDRESS7_1_W<SADDR0_SPEC> {
        ADDRESS7_1_W::new(self, 1)
    }
    #[doc = "Bits 8:9 - Highest two bits of a 10-bit address"]
    #[inline(always)]
    #[must_use]
    pub fn address9_8(&mut self) -> ADDRESS9_8_W<SADDR0_SPEC> {
        ADDRESS9_8_W::new(self, 8)
    }
    #[doc = "Bit 15 - Address mode for the I2C slave"]
    #[inline(always)]
    #[must_use]
    pub fn addformat(&mut self) -> ADDFORMAT_W<SADDR0_SPEC> {
        ADDFORMAT_W::new(self, 15)
    }
}
#[doc = "Slave address register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`saddr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saddr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SADDR0_SPEC;
impl crate::RegisterSpec for SADDR0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`saddr0::R`](R) reader structure"]
impl crate::Readable for SADDR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`saddr0::W`](W) writer structure"]
impl crate::Writable for SADDR0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SADDR0 to value 0"]
impl crate::Resettable for SADDR0_SPEC {
    const RESET_VALUE: u16 = 0;
}
