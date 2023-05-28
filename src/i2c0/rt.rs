#[doc = "Register `RT` reader"]
pub struct R(crate::R<RT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RT` writer"]
pub struct W(crate::W<RT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RT_SPEC>;
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
impl From<crate::W<RT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RISETIME` reader - Maximum rise time in master mode"]
pub type RISETIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RISETIME` writer - Maximum rise time in master mode"]
pub type RISETIME_W<'a, const O: u8> = crate::FieldWriter<'a, u16, RT_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Maximum rise time in master mode"]
    #[inline(always)]
    pub fn risetime(&self) -> RISETIME_R {
        RISETIME_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Maximum rise time in master mode"]
    #[inline(always)]
    #[must_use]
    pub fn risetime(&mut self) -> RISETIME_W<0> {
        RISETIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Rise time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rt](index.html) module"]
pub struct RT_SPEC;
impl crate::RegisterSpec for RT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rt::R](R) reader structure"]
impl crate::Readable for RT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rt::W](W) writer structure"]
impl crate::Writable for RT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RT to value 0x02"]
impl crate::Resettable for RT_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
