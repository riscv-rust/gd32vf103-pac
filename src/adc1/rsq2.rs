#[doc = "Register `RSQ2` reader"]
pub struct R(crate::R<RSQ2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSQ2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSQ2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSQ2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSQ2` writer"]
pub struct W(crate::W<RSQ2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSQ2_SPEC>;
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
impl From<crate::W<RSQ2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSQ2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSQ0` reader - 1st conversion in regular sequence"]
pub type RSQ0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSQ0` writer - 1st conversion in regular sequence"]
pub type RSQ0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RSQ2_SPEC, u8, u8, 5, O>;
#[doc = "Field `RSQ1` reader - 2nd conversion in regular sequence"]
pub type RSQ1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSQ1` writer - 2nd conversion in regular sequence"]
pub type RSQ1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RSQ2_SPEC, u8, u8, 5, O>;
#[doc = "Field `RSQ2` reader - 3rd conversion in regular sequence"]
pub type RSQ2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSQ2` writer - 3rd conversion in regular sequence"]
pub type RSQ2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RSQ2_SPEC, u8, u8, 5, O>;
#[doc = "Field `RSQ3` reader - 4th conversion in regular sequence"]
pub type RSQ3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSQ3` writer - 4th conversion in regular sequence"]
pub type RSQ3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RSQ2_SPEC, u8, u8, 5, O>;
#[doc = "Field `RSQ4` reader - 5th conversion in regular sequence"]
pub type RSQ4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSQ4` writer - 5th conversion in regular sequence"]
pub type RSQ4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RSQ2_SPEC, u8, u8, 5, O>;
#[doc = "Field `RSQ5` reader - 6th conversion in regular sequence"]
pub type RSQ5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSQ5` writer - 6th conversion in regular sequence"]
pub type RSQ5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RSQ2_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - 1st conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq0(&self) -> RSQ0_R {
        RSQ0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 2nd conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq1(&self) -> RSQ1_R {
        RSQ1_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - 3rd conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq2(&self) -> RSQ2_R {
        RSQ2_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - 4th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq3(&self) -> RSQ3_R {
        RSQ3_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - 5th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq4(&self) -> RSQ4_R {
        RSQ4_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - 6th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq5(&self) -> RSQ5_R {
        RSQ5_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 1st conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn rsq0(&mut self) -> RSQ0_W<0> {
        RSQ0_W::new(self)
    }
    #[doc = "Bits 5:9 - 2nd conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn rsq1(&mut self) -> RSQ1_W<5> {
        RSQ1_W::new(self)
    }
    #[doc = "Bits 10:14 - 3rd conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn rsq2(&mut self) -> RSQ2_W<10> {
        RSQ2_W::new(self)
    }
    #[doc = "Bits 15:19 - 4th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn rsq3(&mut self) -> RSQ3_W<15> {
        RSQ3_W::new(self)
    }
    #[doc = "Bits 20:24 - 5th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn rsq4(&mut self) -> RSQ4_W<20> {
        RSQ4_W::new(self)
    }
    #[doc = "Bits 25:29 - 6th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn rsq5(&mut self) -> RSQ5_W<25> {
        RSQ5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "regular sequence register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsq2](index.html) module"]
pub struct RSQ2_SPEC;
impl crate::RegisterSpec for RSQ2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rsq2::R](R) reader structure"]
impl crate::Readable for RSQ2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rsq2::W](W) writer structure"]
impl crate::Writable for RSQ2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RSQ2 to value 0"]
impl crate::Resettable for RSQ2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
