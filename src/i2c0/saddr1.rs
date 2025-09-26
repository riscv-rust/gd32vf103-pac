#[doc = "Register `SADDR1` reader"]
pub type R = crate::R<SADDR1_SPEC>;
#[doc = "Register `SADDR1` writer"]
pub type W = crate::W<SADDR1_SPEC>;
#[doc = "Field `DUADEN` reader - Dual-Address mode switch"]
pub type DUADEN_R = crate::BitReader;
#[doc = "Field `DUADEN` writer - Dual-Address mode switch"]
pub type DUADEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADDRESS2` reader - Second I2C address for the slave in Dual-Address mode"]
pub type ADDRESS2_R = crate::FieldReader;
#[doc = "Field `ADDRESS2` writer - Second I2C address for the slave in Dual-Address mode"]
pub type ADDRESS2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bit 0 - Dual-Address mode switch"]
    #[inline(always)]
    pub fn duaden(&self) -> DUADEN_R {
        DUADEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Second I2C address for the slave in Dual-Address mode"]
    #[inline(always)]
    pub fn address2(&self) -> ADDRESS2_R {
        ADDRESS2_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Dual-Address mode switch"]
    #[inline(always)]
    #[must_use]
    pub fn duaden(&mut self) -> DUADEN_W<SADDR1_SPEC, 0> {
        DUADEN_W::new(self)
    }
    #[doc = "Bits 1:7 - Second I2C address for the slave in Dual-Address mode"]
    #[inline(always)]
    #[must_use]
    pub fn address2(&mut self) -> ADDRESS2_W<SADDR1_SPEC, 1> {
        ADDRESS2_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Slave address register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`saddr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`saddr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SADDR1_SPEC;
impl crate::RegisterSpec for SADDR1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`saddr1::R`](R) reader structure"]
impl crate::Readable for SADDR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`saddr1::W`](W) writer structure"]
impl crate::Writable for SADDR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SADDR1 to value 0"]
impl crate::Resettable for SADDR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
