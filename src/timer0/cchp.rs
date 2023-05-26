#[doc = "Register `CCHP` reader"]
pub struct R(crate::R<CCHP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCHP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCHP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCHP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCHP` writer"]
pub struct W(crate::W<CCHP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCHP_SPEC>;
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
impl From<crate::W<CCHP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCHP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTCFG` reader - Dead time configure"]
pub type DTCFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTCFG` writer - Dead time configure"]
pub type DTCFG_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CCHP_SPEC, u8, u8, 8, O>;
#[doc = "Field `PROT` reader - Complementary register protect control"]
pub type PROT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PROT` writer - Complementary register protect control"]
pub type PROT_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CCHP_SPEC, u8, u8, 2, O>;
#[doc = "Field `IOS` reader - Idle mode off-state configure"]
pub type IOS_R = crate::BitReader<bool>;
#[doc = "Field `IOS` writer - Idle mode off-state configure"]
pub type IOS_W<'a, const O: u8> = crate::BitWriter<'a, u16, CCHP_SPEC, bool, O>;
#[doc = "Field `ROS` reader - Run mode off-state configure"]
pub type ROS_R = crate::BitReader<bool>;
#[doc = "Field `ROS` writer - Run mode off-state configure"]
pub type ROS_W<'a, const O: u8> = crate::BitWriter<'a, u16, CCHP_SPEC, bool, O>;
#[doc = "Field `BRKEN` reader - Break enable"]
pub type BRKEN_R = crate::BitReader<bool>;
#[doc = "Field `BRKEN` writer - Break enable"]
pub type BRKEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CCHP_SPEC, bool, O>;
#[doc = "Field `BRKP` reader - Break polarity"]
pub type BRKP_R = crate::BitReader<bool>;
#[doc = "Field `BRKP` writer - Break polarity"]
pub type BRKP_W<'a, const O: u8> = crate::BitWriter<'a, u16, CCHP_SPEC, bool, O>;
#[doc = "Field `OAEN` reader - Output automatic enable"]
pub type OAEN_R = crate::BitReader<bool>;
#[doc = "Field `OAEN` writer - Output automatic enable"]
pub type OAEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CCHP_SPEC, bool, O>;
#[doc = "Field `POEN` reader - Primary output enable"]
pub type POEN_R = crate::BitReader<bool>;
#[doc = "Field `POEN` writer - Primary output enable"]
pub type POEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CCHP_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Dead time configure"]
    #[inline(always)]
    pub fn dtcfg(&self) -> DTCFG_R {
        DTCFG_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Complementary register protect control"]
    #[inline(always)]
    pub fn prot(&self) -> PROT_R {
        PROT_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Idle mode off-state configure"]
    #[inline(always)]
    pub fn ios(&self) -> IOS_R {
        IOS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Run mode off-state configure"]
    #[inline(always)]
    pub fn ros(&self) -> ROS_R {
        ROS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Break enable"]
    #[inline(always)]
    pub fn brken(&self) -> BRKEN_R {
        BRKEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Break polarity"]
    #[inline(always)]
    pub fn brkp(&self) -> BRKP_R {
        BRKP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Output automatic enable"]
    #[inline(always)]
    pub fn oaen(&self) -> OAEN_R {
        OAEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Primary output enable"]
    #[inline(always)]
    pub fn poen(&self) -> POEN_R {
        POEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Dead time configure"]
    #[inline(always)]
    #[must_use]
    pub fn dtcfg(&mut self) -> DTCFG_W<0> {
        DTCFG_W::new(self)
    }
    #[doc = "Bits 8:9 - Complementary register protect control"]
    #[inline(always)]
    #[must_use]
    pub fn prot(&mut self) -> PROT_W<8> {
        PROT_W::new(self)
    }
    #[doc = "Bit 10 - Idle mode off-state configure"]
    #[inline(always)]
    #[must_use]
    pub fn ios(&mut self) -> IOS_W<10> {
        IOS_W::new(self)
    }
    #[doc = "Bit 11 - Run mode off-state configure"]
    #[inline(always)]
    #[must_use]
    pub fn ros(&mut self) -> ROS_W<11> {
        ROS_W::new(self)
    }
    #[doc = "Bit 12 - Break enable"]
    #[inline(always)]
    #[must_use]
    pub fn brken(&mut self) -> BRKEN_W<12> {
        BRKEN_W::new(self)
    }
    #[doc = "Bit 13 - Break polarity"]
    #[inline(always)]
    #[must_use]
    pub fn brkp(&mut self) -> BRKP_W<13> {
        BRKP_W::new(self)
    }
    #[doc = "Bit 14 - Output automatic enable"]
    #[inline(always)]
    #[must_use]
    pub fn oaen(&mut self) -> OAEN_W<14> {
        OAEN_W::new(self)
    }
    #[doc = "Bit 15 - Primary output enable"]
    #[inline(always)]
    #[must_use]
    pub fn poen(&mut self) -> POEN_W<15> {
        POEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "channel complementary protection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cchp](index.html) module"]
pub struct CCHP_SPEC;
impl crate::RegisterSpec for CCHP_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [cchp::R](R) reader structure"]
impl crate::Readable for CCHP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cchp::W](W) writer structure"]
impl crate::Writable for CCHP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCHP to value 0"]
impl crate::Resettable for CCHP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
