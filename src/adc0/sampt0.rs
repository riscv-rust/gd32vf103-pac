#[doc = "Register `SAMPT0` reader"]
pub struct R(crate::R<SAMPT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAMPT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAMPT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAMPT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAMPT0` writer"]
pub struct W(crate::W<SAMPT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAMPT0_SPEC>;
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
impl From<crate::W<SAMPT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAMPT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPT10` reader - Channel 10 sample time selection"]
pub type SPT10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPT10` writer - Channel 10 sample time selection"]
pub type SPT10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAMPT0_SPEC, u8, u8, 3, O>;
#[doc = "Field `SPT11` reader - Channel 11 sample time selection"]
pub type SPT11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPT11` writer - Channel 11 sample time selection"]
pub type SPT11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAMPT0_SPEC, u8, u8, 3, O>;
#[doc = "Field `SPT12` reader - Channel 12 sample time selection"]
pub type SPT12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPT12` writer - Channel 12 sample time selection"]
pub type SPT12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAMPT0_SPEC, u8, u8, 3, O>;
#[doc = "Field `SPT13` reader - Channel 13 sample time selection"]
pub type SPT13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPT13` writer - Channel 13 sample time selection"]
pub type SPT13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAMPT0_SPEC, u8, u8, 3, O>;
#[doc = "Field `SPT14` reader - Channel 14 sample time selection"]
pub type SPT14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPT14` writer - Channel 14 sample time selection"]
pub type SPT14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAMPT0_SPEC, u8, u8, 3, O>;
#[doc = "Field `SPT15` reader - Channel 15 sample time selection"]
pub type SPT15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPT15` writer - Channel 15 sample time selection"]
pub type SPT15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAMPT0_SPEC, u8, u8, 3, O>;
#[doc = "Field `SPT16` reader - Channel 16 sample time selection"]
pub type SPT16_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPT16` writer - Channel 16 sample time selection"]
pub type SPT16_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAMPT0_SPEC, u8, u8, 3, O>;
#[doc = "Field `SPT17` reader - Channel 17 sample time selection"]
pub type SPT17_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPT17` writer - Channel 17 sample time selection"]
pub type SPT17_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAMPT0_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Channel 10 sample time selection"]
    #[inline(always)]
    pub fn spt10(&self) -> SPT10_R {
        SPT10_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Channel 11 sample time selection"]
    #[inline(always)]
    pub fn spt11(&self) -> SPT11_R {
        SPT11_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Channel 12 sample time selection"]
    #[inline(always)]
    pub fn spt12(&self) -> SPT12_R {
        SPT12_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Channel 13 sample time selection"]
    #[inline(always)]
    pub fn spt13(&self) -> SPT13_R {
        SPT13_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Channel 14 sample time selection"]
    #[inline(always)]
    pub fn spt14(&self) -> SPT14_R {
        SPT14_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Channel 15 sample time selection"]
    #[inline(always)]
    pub fn spt15(&self) -> SPT15_R {
        SPT15_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Channel 16 sample time selection"]
    #[inline(always)]
    pub fn spt16(&self) -> SPT16_R {
        SPT16_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Channel 17 sample time selection"]
    #[inline(always)]
    pub fn spt17(&self) -> SPT17_R {
        SPT17_R::new(((self.bits >> 21) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Channel 10 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt10(&mut self) -> SPT10_W<0> {
        SPT10_W::new(self)
    }
    #[doc = "Bits 3:5 - Channel 11 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt11(&mut self) -> SPT11_W<3> {
        SPT11_W::new(self)
    }
    #[doc = "Bits 6:8 - Channel 12 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt12(&mut self) -> SPT12_W<6> {
        SPT12_W::new(self)
    }
    #[doc = "Bits 9:11 - Channel 13 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt13(&mut self) -> SPT13_W<9> {
        SPT13_W::new(self)
    }
    #[doc = "Bits 12:14 - Channel 14 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt14(&mut self) -> SPT14_W<12> {
        SPT14_W::new(self)
    }
    #[doc = "Bits 15:17 - Channel 15 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt15(&mut self) -> SPT15_W<15> {
        SPT15_W::new(self)
    }
    #[doc = "Bits 18:20 - Channel 16 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt16(&mut self) -> SPT16_W<18> {
        SPT16_W::new(self)
    }
    #[doc = "Bits 21:23 - Channel 17 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt17(&mut self) -> SPT17_W<21> {
        SPT17_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sample time register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sampt0](index.html) module"]
pub struct SAMPT0_SPEC;
impl crate::RegisterSpec for SAMPT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sampt0::R](R) reader structure"]
impl crate::Readable for SAMPT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sampt0::W](W) writer structure"]
impl crate::Writable for SAMPT0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAMPT0 to value 0"]
impl crate::Resettable for SAMPT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
