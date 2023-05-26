#[doc = "Register `DIEPFEINTEN` reader"]
pub struct R(crate::R<DIEPFEINTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPFEINTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPFEINTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPFEINTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEPFEINTEN` writer"]
pub struct W(crate::W<DIEPFEINTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPFEINTEN_SPEC>;
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
impl From<crate::W<DIEPFEINTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPFEINTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IEPTXFEIE` reader - IN EP Tx FIFO empty interrupt enable bits"]
pub type IEPTXFEIE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IEPTXFEIE` writer - IN EP Tx FIFO empty interrupt enable bits"]
pub type IEPTXFEIE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIEPFEINTEN_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - IN EP Tx FIFO empty interrupt enable bits"]
    #[inline(always)]
    pub fn ieptxfeie(&self) -> IEPTXFEIE_R {
        IEPTXFEIE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - IN EP Tx FIFO empty interrupt enable bits"]
    #[inline(always)]
    #[must_use]
    pub fn ieptxfeie(&mut self) -> IEPTXFEIE_W<0> {
        IEPTXFEIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "device IN endpoint FIFO empty interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepfeinten](index.html) module"]
pub struct DIEPFEINTEN_SPEC;
impl crate::RegisterSpec for DIEPFEINTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diepfeinten::R](R) reader structure"]
impl crate::Readable for DIEPFEINTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [diepfeinten::W](W) writer structure"]
impl crate::Writable for DIEPFEINTEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPFEINTEN to value 0"]
impl crate::Resettable for DIEPFEINTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
