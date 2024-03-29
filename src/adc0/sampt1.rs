#[doc = "Register `SAMPT1` reader"]
pub struct R(crate::R<SAMPT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAMPT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAMPT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAMPT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAMPT1` writer"]
pub struct W(crate::W<SAMPT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAMPT1_SPEC>;
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
impl From<crate::W<SAMPT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAMPT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPT0` reader - Channel 0 sample time selection"]
pub type SPT0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPT0` writer - Channel 0 sample time selection"]
pub type SPT0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAMPT1_SPEC, u8, u8, 3, O>;
#[doc = "Field `SPT1` reader - Channel 1 sample time selection"]
pub type SPT1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPT1` writer - Channel 1 sample time selection"]
pub type SPT1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAMPT1_SPEC, u8, u8, 3, O>;
#[doc = "Field `SPT2` reader - Channel 2 sample time selection"]
pub type SPT2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPT2` writer - Channel 2 sample time selection"]
pub type SPT2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAMPT1_SPEC, u8, u8, 3, O>;
#[doc = "Field `SPT3` reader - Channel 3 sample time selection"]
pub type SPT3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPT3` writer - Channel 3 sample time selection"]
pub type SPT3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAMPT1_SPEC, u8, u8, 3, O>;
#[doc = "Field `SPT4` reader - Channel 4 sample time selection"]
pub type SPT4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPT4` writer - Channel 4 sample time selection"]
pub type SPT4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAMPT1_SPEC, u8, u8, 3, O>;
#[doc = "Field `SPT5` reader - Channel 5 sample time selection"]
pub type SPT5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPT5` writer - Channel 5 sample time selection"]
pub type SPT5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAMPT1_SPEC, u8, u8, 3, O>;
#[doc = "Field `SPT6` reader - Channel 6 sample time selection"]
pub type SPT6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPT6` writer - Channel 6 sample time selection"]
pub type SPT6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAMPT1_SPEC, u8, u8, 3, O>;
#[doc = "Field `SPT7` reader - Channel 7 sample time selection"]
pub type SPT7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPT7` writer - Channel 7 sample time selection"]
pub type SPT7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAMPT1_SPEC, u8, u8, 3, O>;
#[doc = "Field `SPT8` reader - Channel 8 sample time selection"]
pub type SPT8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPT8` writer - Channel 8 sample time selection"]
pub type SPT8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAMPT1_SPEC, u8, u8, 3, O>;
#[doc = "Field `SPT9` reader - Channel 9 sample time selection"]
pub type SPT9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPT9` writer - Channel 9 sample time selection"]
pub type SPT9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAMPT1_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Channel 0 sample time selection"]
    #[inline(always)]
    pub fn spt0(&self) -> SPT0_R {
        SPT0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Channel 1 sample time selection"]
    #[inline(always)]
    pub fn spt1(&self) -> SPT1_R {
        SPT1_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Channel 2 sample time selection"]
    #[inline(always)]
    pub fn spt2(&self) -> SPT2_R {
        SPT2_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Channel 3 sample time selection"]
    #[inline(always)]
    pub fn spt3(&self) -> SPT3_R {
        SPT3_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Channel 4 sample time selection"]
    #[inline(always)]
    pub fn spt4(&self) -> SPT4_R {
        SPT4_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Channel 5 sample time selection"]
    #[inline(always)]
    pub fn spt5(&self) -> SPT5_R {
        SPT5_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Channel 6 sample time selection"]
    #[inline(always)]
    pub fn spt6(&self) -> SPT6_R {
        SPT6_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Channel 7 sample time selection"]
    #[inline(always)]
    pub fn spt7(&self) -> SPT7_R {
        SPT7_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Channel 8 sample time selection"]
    #[inline(always)]
    pub fn spt8(&self) -> SPT8_R {
        SPT8_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - Channel 9 sample time selection"]
    #[inline(always)]
    pub fn spt9(&self) -> SPT9_R {
        SPT9_R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Channel 0 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt0(&mut self) -> SPT0_W<0> {
        SPT0_W::new(self)
    }
    #[doc = "Bits 3:5 - Channel 1 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt1(&mut self) -> SPT1_W<3> {
        SPT1_W::new(self)
    }
    #[doc = "Bits 6:8 - Channel 2 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt2(&mut self) -> SPT2_W<6> {
        SPT2_W::new(self)
    }
    #[doc = "Bits 9:11 - Channel 3 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt3(&mut self) -> SPT3_W<9> {
        SPT3_W::new(self)
    }
    #[doc = "Bits 12:14 - Channel 4 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt4(&mut self) -> SPT4_W<12> {
        SPT4_W::new(self)
    }
    #[doc = "Bits 15:17 - Channel 5 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt5(&mut self) -> SPT5_W<15> {
        SPT5_W::new(self)
    }
    #[doc = "Bits 18:20 - Channel 6 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt6(&mut self) -> SPT6_W<18> {
        SPT6_W::new(self)
    }
    #[doc = "Bits 21:23 - Channel 7 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt7(&mut self) -> SPT7_W<21> {
        SPT7_W::new(self)
    }
    #[doc = "Bits 24:26 - Channel 8 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt8(&mut self) -> SPT8_W<24> {
        SPT8_W::new(self)
    }
    #[doc = "Bits 27:29 - Channel 9 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt9(&mut self) -> SPT9_W<27> {
        SPT9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sample time register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sampt1](index.html) module"]
pub struct SAMPT1_SPEC;
impl crate::RegisterSpec for SAMPT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sampt1::R](R) reader structure"]
impl crate::Readable for SAMPT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sampt1::W](W) writer structure"]
impl crate::Writable for SAMPT1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAMPT1 to value 0"]
impl crate::Resettable for SAMPT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
