#[doc = "Register `DVBUSDT` reader"]
pub struct R(crate::R<DVBUSDT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DVBUSDT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DVBUSDT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DVBUSDT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DVBUSDT` writer"]
pub struct W(crate::W<DVBUSDT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DVBUSDT_SPEC>;
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
impl From<crate::W<DVBUSDT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DVBUSDT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DVBUSDT` reader - Device VBUS discharge time"]
pub type DVBUSDT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DVBUSDT` writer - Device VBUS discharge time"]
pub type DVBUSDT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DVBUSDT_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Device VBUS discharge time"]
    #[inline(always)]
    pub fn dvbusdt(&self) -> DVBUSDT_R {
        DVBUSDT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Device VBUS discharge time"]
    #[inline(always)]
    #[must_use]
    pub fn dvbusdt(&mut self) -> DVBUSDT_W<0> {
        DVBUSDT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "device VBUS discharge time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dvbusdt](index.html) module"]
pub struct DVBUSDT_SPEC;
impl crate::RegisterSpec for DVBUSDT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dvbusdt::R](R) reader structure"]
impl crate::Readable for DVBUSDT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dvbusdt::W](W) writer structure"]
impl crate::Writable for DVBUSDT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DVBUSDT to value 0x17d7"]
impl crate::Resettable for DVBUSDT_SPEC {
    const RESET_VALUE: Self::Ux = 0x17d7;
}
