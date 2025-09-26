#[doc = "Register `DVBUSPT` reader"]
pub type R = crate::R<DVBUSPT_SPEC>;
#[doc = "Register `DVBUSPT` writer"]
pub type W = crate::W<DVBUSPT_SPEC>;
#[doc = "Field `DVBUSPT` reader - Device VBUS pulsing time"]
pub type DVBUSPT_R = crate::FieldReader<u16>;
#[doc = "Field `DVBUSPT` writer - Device VBUS pulsing time"]
pub type DVBUSPT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - Device VBUS pulsing time"]
    #[inline(always)]
    pub fn dvbuspt(&self) -> DVBUSPT_R {
        DVBUSPT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Device VBUS pulsing time"]
    #[inline(always)]
    #[must_use]
    pub fn dvbuspt(&mut self) -> DVBUSPT_W<DVBUSPT_SPEC, 0> {
        DVBUSPT_W::new(self)
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
#[doc = "device VBUS pulsing time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dvbuspt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dvbuspt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DVBUSPT_SPEC;
impl crate::RegisterSpec for DVBUSPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dvbuspt::R`](R) reader structure"]
impl crate::Readable for DVBUSPT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dvbuspt::W`](W) writer structure"]
impl crate::Writable for DVBUSPT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DVBUSPT to value 0x05b8"]
impl crate::Resettable for DVBUSPT_SPEC {
    const RESET_VALUE: Self::Ux = 0x05b8;
}
