#[doc = "Register `WDLT` reader"]
pub type R = crate::R<WDLT_SPEC>;
#[doc = "Register `WDLT` writer"]
pub type W = crate::W<WDLT_SPEC>;
#[doc = "Field `WDLT` reader - Analog watchdog lower threshold"]
pub type WDLT_R = crate::FieldReader<u16>;
#[doc = "Field `WDLT` writer - Analog watchdog lower threshold"]
pub type WDLT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - Analog watchdog lower threshold"]
    #[inline(always)]
    pub fn wdlt(&self) -> WDLT_R {
        WDLT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Analog watchdog lower threshold"]
    #[inline(always)]
    #[must_use]
    pub fn wdlt(&mut self) -> WDLT_W<WDLT_SPEC, 0> {
        WDLT_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "watchdog lower threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdlt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdlt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDLT_SPEC;
impl crate::RegisterSpec for WDLT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdlt::R`](R) reader structure"]
impl crate::Readable for WDLT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdlt::W`](W) writer structure"]
impl crate::Writable for WDLT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDLT to value 0"]
impl crate::Resettable for WDLT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
