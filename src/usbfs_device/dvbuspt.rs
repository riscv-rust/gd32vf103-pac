#[doc = "Register `DVBUSPT` reader"]
pub struct R(crate::R<DVBUSPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DVBUSPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DVBUSPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DVBUSPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DVBUSPT` writer"]
pub struct W(crate::W<DVBUSPT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DVBUSPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DVBUSPT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DVBUSPT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DVBUSPT` reader - Device VBUS pulsing time"]
pub type DVBUSPT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DVBUSPT` writer - Device VBUS pulsing time"]
pub type DVBUSPT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DVBUSPT_SPEC, u16, u16, 12, O>;
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
    pub fn dvbuspt(&mut self) -> DVBUSPT_W<0> {
        DVBUSPT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "device VBUS pulsing time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dvbuspt](index.html) module"]
pub struct DVBUSPT_SPEC;
impl crate::RegisterSpec for DVBUSPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dvbuspt::R](R) reader structure"]
impl crate::Readable for DVBUSPT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dvbuspt::W](W) writer structure"]
impl crate::Writable for DVBUSPT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DVBUSPT to value 0x05b8"]
impl crate::Resettable for DVBUSPT_SPEC {
    const RESET_VALUE: Self::Ux = 0x05b8;
}
