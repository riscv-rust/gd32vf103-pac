#[doc = "Register `I2SPSC` reader"]
pub type R = crate::R<I2SPSC_SPEC>;
#[doc = "Register `I2SPSC` writer"]
pub type W = crate::W<I2SPSC_SPEC>;
#[doc = "Field `DIV` reader - Dividing factor for the prescaler"]
pub type DIV_R = crate::FieldReader;
#[doc = "Field `DIV` writer - Dividing factor for the prescaler"]
pub type DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `OF` reader - Odd factor for the prescaler"]
pub type OF_R = crate::BitReader;
#[doc = "Field `OF` writer - Odd factor for the prescaler"]
pub type OF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCKOEN` reader - I2S_MCK output enable"]
pub type MCKOEN_R = crate::BitReader;
#[doc = "Field `MCKOEN` writer - I2S_MCK output enable"]
pub type MCKOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Dividing factor for the prescaler"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Odd factor for the prescaler"]
    #[inline(always)]
    pub fn of(&self) -> OF_R {
        OF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - I2S_MCK output enable"]
    #[inline(always)]
    pub fn mckoen(&self) -> MCKOEN_R {
        MCKOEN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Dividing factor for the prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<I2SPSC_SPEC> {
        DIV_W::new(self, 0)
    }
    #[doc = "Bit 8 - Odd factor for the prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn of(&mut self) -> OF_W<I2SPSC_SPEC> {
        OF_W::new(self, 8)
    }
    #[doc = "Bit 9 - I2S_MCK output enable"]
    #[inline(always)]
    #[must_use]
    pub fn mckoen(&mut self) -> MCKOEN_W<I2SPSC_SPEC> {
        MCKOEN_W::new(self, 9)
    }
}
#[doc = "I2S prescaler register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2spsc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2spsc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2SPSC_SPEC;
impl crate::RegisterSpec for I2SPSC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`i2spsc::R`](R) reader structure"]
impl crate::Readable for I2SPSC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2spsc::W`](W) writer structure"]
impl crate::Writable for I2SPSC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets I2SPSC to value 0x02"]
impl crate::Resettable for I2SPSC_SPEC {
    const RESET_VALUE: u16 = 0x02;
}
