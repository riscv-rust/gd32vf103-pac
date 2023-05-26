#[doc = "Register `HCTL` reader"]
pub struct R(crate::R<HCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCTL` writer"]
pub struct W(crate::W<HCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCTL_SPEC>;
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
impl From<crate::W<HCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKSEL` reader - clock select for USB clock"]
pub type CLKSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLKSEL` writer - clock select for USB clock"]
pub type CLKSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCTL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - clock select for USB clock"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - clock select for USB clock"]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> CLKSEL_W<0> {
        CLKSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "host configuration register (HCTL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hctl](index.html) module"]
pub struct HCTL_SPEC;
impl crate::RegisterSpec for HCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hctl::R](R) reader structure"]
impl crate::Readable for HCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hctl::W](W) writer structure"]
impl crate::Writable for HCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCTL to value 0"]
impl crate::Resettable for HCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
