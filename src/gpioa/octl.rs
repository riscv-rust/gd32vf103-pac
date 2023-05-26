#[doc = "Register `OCTL` reader"]
pub struct R(crate::R<OCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OCTL` writer"]
pub struct W(crate::W<OCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OCTL_SPEC>;
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
impl From<crate::W<OCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OCTL0` reader - Port output control"]
pub type OCTL0_R = crate::BitReader<bool>;
#[doc = "Field `OCTL0` writer - Port output control"]
pub type OCTL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCTL_SPEC, bool, O>;
#[doc = "Field `OCTL1` reader - Port output control"]
pub type OCTL1_R = crate::BitReader<bool>;
#[doc = "Field `OCTL1` writer - Port output control"]
pub type OCTL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCTL_SPEC, bool, O>;
#[doc = "Field `OCTL2` reader - Port output control"]
pub type OCTL2_R = crate::BitReader<bool>;
#[doc = "Field `OCTL2` writer - Port output control"]
pub type OCTL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCTL_SPEC, bool, O>;
#[doc = "Field `OCTL3` reader - Port output control"]
pub type OCTL3_R = crate::BitReader<bool>;
#[doc = "Field `OCTL3` writer - Port output control"]
pub type OCTL3_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCTL_SPEC, bool, O>;
#[doc = "Field `OCTL4` reader - Port output control"]
pub type OCTL4_R = crate::BitReader<bool>;
#[doc = "Field `OCTL4` writer - Port output control"]
pub type OCTL4_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCTL_SPEC, bool, O>;
#[doc = "Field `OCTL5` reader - Port output control"]
pub type OCTL5_R = crate::BitReader<bool>;
#[doc = "Field `OCTL5` writer - Port output control"]
pub type OCTL5_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCTL_SPEC, bool, O>;
#[doc = "Field `OCTL6` reader - Port output control"]
pub type OCTL6_R = crate::BitReader<bool>;
#[doc = "Field `OCTL6` writer - Port output control"]
pub type OCTL6_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCTL_SPEC, bool, O>;
#[doc = "Field `OCTL7` reader - Port output control"]
pub type OCTL7_R = crate::BitReader<bool>;
#[doc = "Field `OCTL7` writer - Port output control"]
pub type OCTL7_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCTL_SPEC, bool, O>;
#[doc = "Field `OCTL8` reader - Port output control"]
pub type OCTL8_R = crate::BitReader<bool>;
#[doc = "Field `OCTL8` writer - Port output control"]
pub type OCTL8_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCTL_SPEC, bool, O>;
#[doc = "Field `OCTL9` reader - Port output control"]
pub type OCTL9_R = crate::BitReader<bool>;
#[doc = "Field `OCTL9` writer - Port output control"]
pub type OCTL9_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCTL_SPEC, bool, O>;
#[doc = "Field `OCTL10` reader - Port output control"]
pub type OCTL10_R = crate::BitReader<bool>;
#[doc = "Field `OCTL10` writer - Port output control"]
pub type OCTL10_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCTL_SPEC, bool, O>;
#[doc = "Field `OCTL11` reader - Port output control"]
pub type OCTL11_R = crate::BitReader<bool>;
#[doc = "Field `OCTL11` writer - Port output control"]
pub type OCTL11_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCTL_SPEC, bool, O>;
#[doc = "Field `OCTL12` reader - Port output control"]
pub type OCTL12_R = crate::BitReader<bool>;
#[doc = "Field `OCTL12` writer - Port output control"]
pub type OCTL12_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCTL_SPEC, bool, O>;
#[doc = "Field `OCTL13` reader - Port output control"]
pub type OCTL13_R = crate::BitReader<bool>;
#[doc = "Field `OCTL13` writer - Port output control"]
pub type OCTL13_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCTL_SPEC, bool, O>;
#[doc = "Field `OCTL14` reader - Port output control"]
pub type OCTL14_R = crate::BitReader<bool>;
#[doc = "Field `OCTL14` writer - Port output control"]
pub type OCTL14_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCTL_SPEC, bool, O>;
#[doc = "Field `OCTL15` reader - Port output control"]
pub type OCTL15_R = crate::BitReader<bool>;
#[doc = "Field `OCTL15` writer - Port output control"]
pub type OCTL15_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Port output control"]
    #[inline(always)]
    pub fn octl0(&self) -> OCTL0_R {
        OCTL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port output control"]
    #[inline(always)]
    pub fn octl1(&self) -> OCTL1_R {
        OCTL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port output control"]
    #[inline(always)]
    pub fn octl2(&self) -> OCTL2_R {
        OCTL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port output control"]
    #[inline(always)]
    pub fn octl3(&self) -> OCTL3_R {
        OCTL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port output control"]
    #[inline(always)]
    pub fn octl4(&self) -> OCTL4_R {
        OCTL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port output control"]
    #[inline(always)]
    pub fn octl5(&self) -> OCTL5_R {
        OCTL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port output control"]
    #[inline(always)]
    pub fn octl6(&self) -> OCTL6_R {
        OCTL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port output control"]
    #[inline(always)]
    pub fn octl7(&self) -> OCTL7_R {
        OCTL7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port output control"]
    #[inline(always)]
    pub fn octl8(&self) -> OCTL8_R {
        OCTL8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port output control"]
    #[inline(always)]
    pub fn octl9(&self) -> OCTL9_R {
        OCTL9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port output control"]
    #[inline(always)]
    pub fn octl10(&self) -> OCTL10_R {
        OCTL10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port output control"]
    #[inline(always)]
    pub fn octl11(&self) -> OCTL11_R {
        OCTL11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port output control"]
    #[inline(always)]
    pub fn octl12(&self) -> OCTL12_R {
        OCTL12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port output control"]
    #[inline(always)]
    pub fn octl13(&self) -> OCTL13_R {
        OCTL13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port output control"]
    #[inline(always)]
    pub fn octl14(&self) -> OCTL14_R {
        OCTL14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port output control"]
    #[inline(always)]
    pub fn octl15(&self) -> OCTL15_R {
        OCTL15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port output control"]
    #[inline(always)]
    #[must_use]
    pub fn octl0(&mut self) -> OCTL0_W<0> {
        OCTL0_W::new(self)
    }
    #[doc = "Bit 1 - Port output control"]
    #[inline(always)]
    #[must_use]
    pub fn octl1(&mut self) -> OCTL1_W<1> {
        OCTL1_W::new(self)
    }
    #[doc = "Bit 2 - Port output control"]
    #[inline(always)]
    #[must_use]
    pub fn octl2(&mut self) -> OCTL2_W<2> {
        OCTL2_W::new(self)
    }
    #[doc = "Bit 3 - Port output control"]
    #[inline(always)]
    #[must_use]
    pub fn octl3(&mut self) -> OCTL3_W<3> {
        OCTL3_W::new(self)
    }
    #[doc = "Bit 4 - Port output control"]
    #[inline(always)]
    #[must_use]
    pub fn octl4(&mut self) -> OCTL4_W<4> {
        OCTL4_W::new(self)
    }
    #[doc = "Bit 5 - Port output control"]
    #[inline(always)]
    #[must_use]
    pub fn octl5(&mut self) -> OCTL5_W<5> {
        OCTL5_W::new(self)
    }
    #[doc = "Bit 6 - Port output control"]
    #[inline(always)]
    #[must_use]
    pub fn octl6(&mut self) -> OCTL6_W<6> {
        OCTL6_W::new(self)
    }
    #[doc = "Bit 7 - Port output control"]
    #[inline(always)]
    #[must_use]
    pub fn octl7(&mut self) -> OCTL7_W<7> {
        OCTL7_W::new(self)
    }
    #[doc = "Bit 8 - Port output control"]
    #[inline(always)]
    #[must_use]
    pub fn octl8(&mut self) -> OCTL8_W<8> {
        OCTL8_W::new(self)
    }
    #[doc = "Bit 9 - Port output control"]
    #[inline(always)]
    #[must_use]
    pub fn octl9(&mut self) -> OCTL9_W<9> {
        OCTL9_W::new(self)
    }
    #[doc = "Bit 10 - Port output control"]
    #[inline(always)]
    #[must_use]
    pub fn octl10(&mut self) -> OCTL10_W<10> {
        OCTL10_W::new(self)
    }
    #[doc = "Bit 11 - Port output control"]
    #[inline(always)]
    #[must_use]
    pub fn octl11(&mut self) -> OCTL11_W<11> {
        OCTL11_W::new(self)
    }
    #[doc = "Bit 12 - Port output control"]
    #[inline(always)]
    #[must_use]
    pub fn octl12(&mut self) -> OCTL12_W<12> {
        OCTL12_W::new(self)
    }
    #[doc = "Bit 13 - Port output control"]
    #[inline(always)]
    #[must_use]
    pub fn octl13(&mut self) -> OCTL13_W<13> {
        OCTL13_W::new(self)
    }
    #[doc = "Bit 14 - Port output control"]
    #[inline(always)]
    #[must_use]
    pub fn octl14(&mut self) -> OCTL14_W<14> {
        OCTL14_W::new(self)
    }
    #[doc = "Bit 15 - Port output control"]
    #[inline(always)]
    #[must_use]
    pub fn octl15(&mut self) -> OCTL15_W<15> {
        OCTL15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port output control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [octl](index.html) module"]
pub struct OCTL_SPEC;
impl crate::RegisterSpec for OCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [octl::R](R) reader structure"]
impl crate::Readable for OCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [octl::W](W) writer structure"]
impl crate::Writable for OCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCTL to value 0"]
impl crate::Resettable for OCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
