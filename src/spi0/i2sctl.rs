#[doc = "Register `I2SCTL` reader"]
pub type R = crate::R<I2SCTL_SPEC>;
#[doc = "Register `I2SCTL` writer"]
pub type W = crate::W<I2SCTL_SPEC>;
#[doc = "Field `CHLEN` reader - Channel length (number of bits per audio channel)"]
pub type CHLEN_R = crate::BitReader;
#[doc = "Field `CHLEN` writer - Channel length (number of bits per audio channel)"]
pub type CHLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTLEN` reader - Data length"]
pub type DTLEN_R = crate::FieldReader;
#[doc = "Field `DTLEN` writer - Data length"]
pub type DTLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CKPL` reader - Idle state clock polarity"]
pub type CKPL_R = crate::BitReader;
#[doc = "Field `CKPL` writer - Idle state clock polarity"]
pub type CKPL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2SSTD` reader - I2S standard selection"]
pub type I2SSTD_R = crate::FieldReader;
#[doc = "Field `I2SSTD` writer - I2S standard selection"]
pub type I2SSTD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PCMSMOD` reader - PCM frame synchronization mode"]
pub type PCMSMOD_R = crate::BitReader;
#[doc = "Field `PCMSMOD` writer - PCM frame synchronization mode"]
pub type PCMSMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2SOPMOD` reader - I2S operation mode"]
pub type I2SOPMOD_R = crate::FieldReader;
#[doc = "Field `I2SOPMOD` writer - I2S operation mode"]
pub type I2SOPMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2SEN` reader - I2S Enable"]
pub type I2SEN_R = crate::BitReader;
#[doc = "Field `I2SEN` writer - I2S Enable"]
pub type I2SEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2SSEL` reader - I2S mode selection"]
pub type I2SSEL_R = crate::BitReader;
#[doc = "Field `I2SSEL` writer - I2S mode selection"]
pub type I2SSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel length (number of bits per audio channel)"]
    #[inline(always)]
    pub fn chlen(&self) -> CHLEN_R {
        CHLEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Data length"]
    #[inline(always)]
    pub fn dtlen(&self) -> DTLEN_R {
        DTLEN_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Idle state clock polarity"]
    #[inline(always)]
    pub fn ckpl(&self) -> CKPL_R {
        CKPL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - I2S standard selection"]
    #[inline(always)]
    pub fn i2sstd(&self) -> I2SSTD_R {
        I2SSTD_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - PCM frame synchronization mode"]
    #[inline(always)]
    pub fn pcmsmod(&self) -> PCMSMOD_R {
        PCMSMOD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - I2S operation mode"]
    #[inline(always)]
    pub fn i2sopmod(&self) -> I2SOPMOD_R {
        I2SOPMOD_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - I2S Enable"]
    #[inline(always)]
    pub fn i2sen(&self) -> I2SEN_R {
        I2SEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - I2S mode selection"]
    #[inline(always)]
    pub fn i2ssel(&self) -> I2SSEL_R {
        I2SSEL_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel length (number of bits per audio channel)"]
    #[inline(always)]
    #[must_use]
    pub fn chlen(&mut self) -> CHLEN_W<I2SCTL_SPEC> {
        CHLEN_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Data length"]
    #[inline(always)]
    #[must_use]
    pub fn dtlen(&mut self) -> DTLEN_W<I2SCTL_SPEC> {
        DTLEN_W::new(self, 1)
    }
    #[doc = "Bit 3 - Idle state clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ckpl(&mut self) -> CKPL_W<I2SCTL_SPEC> {
        CKPL_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - I2S standard selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2sstd(&mut self) -> I2SSTD_W<I2SCTL_SPEC> {
        I2SSTD_W::new(self, 4)
    }
    #[doc = "Bit 7 - PCM frame synchronization mode"]
    #[inline(always)]
    #[must_use]
    pub fn pcmsmod(&mut self) -> PCMSMOD_W<I2SCTL_SPEC> {
        PCMSMOD_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - I2S operation mode"]
    #[inline(always)]
    #[must_use]
    pub fn i2sopmod(&mut self) -> I2SOPMOD_W<I2SCTL_SPEC> {
        I2SOPMOD_W::new(self, 8)
    }
    #[doc = "Bit 10 - I2S Enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2sen(&mut self) -> I2SEN_W<I2SCTL_SPEC> {
        I2SEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - I2S mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2ssel(&mut self) -> I2SSEL_W<I2SCTL_SPEC> {
        I2SSEL_W::new(self, 11)
    }
}
#[doc = "I2S control register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2sctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2sctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2SCTL_SPEC;
impl crate::RegisterSpec for I2SCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`i2sctl::R`](R) reader structure"]
impl crate::Readable for I2SCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2sctl::W`](W) writer structure"]
impl crate::Writable for I2SCTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets I2SCTL to value 0"]
impl crate::Resettable for I2SCTL_SPEC {
    const RESET_VALUE: u16 = 0;
}
