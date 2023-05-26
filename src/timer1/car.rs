#[doc = "Register `CAR` reader"]
pub struct R(crate::R<CAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAR` writer"]
pub struct W(crate::W<CAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAR_SPEC>;
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
impl From<crate::W<CAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CARL` reader - Counter auto reload value"]
pub type CARL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CARL` writer - Counter auto reload value"]
pub type CARL_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CAR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Counter auto reload value"]
    #[inline(always)]
    pub fn carl(&self) -> CARL_R {
        CARL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Counter auto reload value"]
    #[inline(always)]
    #[must_use]
    pub fn carl(&mut self) -> CARL_W<0> {
        CARL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter auto reload register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [car](index.html) module"]
pub struct CAR_SPEC;
impl crate::RegisterSpec for CAR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [car::R](R) reader structure"]
impl crate::Readable for CAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [car::W](W) writer structure"]
impl crate::Writable for CAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAR to value 0"]
impl crate::Resettable for CAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
