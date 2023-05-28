#[doc = "Register `CLICINTIP` reader"]
pub struct R(crate::R<CLICINTIP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLICINTIP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLICINTIP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLICINTIP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLICINTIP` writer"]
pub struct W(crate::W<CLICINTIP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLICINTIP_SPEC>;
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
impl From<crate::W<CLICINTIP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLICINTIP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IP` reader - IP"]
pub type IP_R = crate::BitReader<bool>;
#[doc = "Field `IP` writer - IP"]
pub type IP_W<'a, const O: u8> = crate::BitWriter<'a, u8, CLICINTIP_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - IP"]
    #[inline(always)]
    pub fn ip(&self) -> IP_R {
        IP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IP"]
    #[inline(always)]
    #[must_use]
    pub fn ip(&mut self) -> IP_W<0> {
        IP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "clicintip Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clicintip](index.html) module"]
pub struct CLICINTIP_SPEC;
impl crate::RegisterSpec for CLICINTIP_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [clicintip::R](R) reader structure"]
impl crate::Readable for CLICINTIP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clicintip::W](W) writer structure"]
impl crate::Writable for CLICINTIP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLICINTIP to value 0"]
impl crate::Resettable for CLICINTIP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
