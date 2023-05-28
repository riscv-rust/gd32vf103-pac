#[doc = "Register `msip` reader"]
pub struct R(crate::R<MSIP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSIP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSIP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSIP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `msip` writer"]
pub struct W(crate::W<MSIP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSIP_SPEC>;
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
impl From<crate::W<MSIP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSIP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSIP` reader - Generate software interrupts"]
pub type MSIP_R = crate::BitReader<bool>;
#[doc = "Field `MSIP` writer - Generate software interrupts"]
pub type MSIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSIP_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Generate software interrupts"]
    #[inline(always)]
    pub fn msip(&self) -> MSIP_R {
        MSIP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Generate software interrupts"]
    #[inline(always)]
    #[must_use]
    pub fn msip(&mut self) -> MSIP_W<0> {
        MSIP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msip](index.html) module"]
pub struct MSIP_SPEC;
impl crate::RegisterSpec for MSIP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msip::R](R) reader structure"]
impl crate::Readable for MSIP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [msip::W](W) writer structure"]
impl crate::Writable for MSIP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets msip to value 0"]
impl crate::Resettable for MSIP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
