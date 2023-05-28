#[doc = "Register `RTEN` reader"]
pub struct R(crate::R<RTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTEN` writer"]
pub struct W(crate::W<RTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTEN_SPEC>;
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
impl From<crate::W<RTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTEN0` reader - Rising edge trigger enable of line 0"]
pub type RTEN0_R = crate::BitReader<bool>;
#[doc = "Field `RTEN0` writer - Rising edge trigger enable of line 0"]
pub type RTEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTEN_SPEC, bool, O>;
#[doc = "Field `RTEN1` reader - Rising edge trigger enable of line 1"]
pub type RTEN1_R = crate::BitReader<bool>;
#[doc = "Field `RTEN1` writer - Rising edge trigger enable of line 1"]
pub type RTEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTEN_SPEC, bool, O>;
#[doc = "Field `RTEN2` reader - Rising edge trigger enable of line 2"]
pub type RTEN2_R = crate::BitReader<bool>;
#[doc = "Field `RTEN2` writer - Rising edge trigger enable of line 2"]
pub type RTEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTEN_SPEC, bool, O>;
#[doc = "Field `RTEN3` reader - Rising edge trigger enable of line 3"]
pub type RTEN3_R = crate::BitReader<bool>;
#[doc = "Field `RTEN3` writer - Rising edge trigger enable of line 3"]
pub type RTEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTEN_SPEC, bool, O>;
#[doc = "Field `RTEN4` reader - Rising edge trigger enable of line 4"]
pub type RTEN4_R = crate::BitReader<bool>;
#[doc = "Field `RTEN4` writer - Rising edge trigger enable of line 4"]
pub type RTEN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTEN_SPEC, bool, O>;
#[doc = "Field `RTEN5` reader - Rising edge trigger enable of line 5"]
pub type RTEN5_R = crate::BitReader<bool>;
#[doc = "Field `RTEN5` writer - Rising edge trigger enable of line 5"]
pub type RTEN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTEN_SPEC, bool, O>;
#[doc = "Field `RTEN6` reader - Rising edge trigger enable of line 6"]
pub type RTEN6_R = crate::BitReader<bool>;
#[doc = "Field `RTEN6` writer - Rising edge trigger enable of line 6"]
pub type RTEN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTEN_SPEC, bool, O>;
#[doc = "Field `RTEN7` reader - Rising edge trigger enable of line 7"]
pub type RTEN7_R = crate::BitReader<bool>;
#[doc = "Field `RTEN7` writer - Rising edge trigger enable of line 7"]
pub type RTEN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTEN_SPEC, bool, O>;
#[doc = "Field `RTEN8` reader - Rising edge trigger enable of line 8"]
pub type RTEN8_R = crate::BitReader<bool>;
#[doc = "Field `RTEN8` writer - Rising edge trigger enable of line 8"]
pub type RTEN8_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTEN_SPEC, bool, O>;
#[doc = "Field `RTEN9` reader - Rising edge trigger enable of line 9"]
pub type RTEN9_R = crate::BitReader<bool>;
#[doc = "Field `RTEN9` writer - Rising edge trigger enable of line 9"]
pub type RTEN9_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTEN_SPEC, bool, O>;
#[doc = "Field `RTEN10` reader - Rising edge trigger enable of line 10"]
pub type RTEN10_R = crate::BitReader<bool>;
#[doc = "Field `RTEN10` writer - Rising edge trigger enable of line 10"]
pub type RTEN10_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTEN_SPEC, bool, O>;
#[doc = "Field `RTEN11` reader - Rising edge trigger enable of line 11"]
pub type RTEN11_R = crate::BitReader<bool>;
#[doc = "Field `RTEN11` writer - Rising edge trigger enable of line 11"]
pub type RTEN11_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTEN_SPEC, bool, O>;
#[doc = "Field `RTEN12` reader - Rising edge trigger enable of line 12"]
pub type RTEN12_R = crate::BitReader<bool>;
#[doc = "Field `RTEN12` writer - Rising edge trigger enable of line 12"]
pub type RTEN12_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTEN_SPEC, bool, O>;
#[doc = "Field `RTEN13` reader - Rising edge trigger enable of line 13"]
pub type RTEN13_R = crate::BitReader<bool>;
#[doc = "Field `RTEN13` writer - Rising edge trigger enable of line 13"]
pub type RTEN13_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTEN_SPEC, bool, O>;
#[doc = "Field `RTEN14` reader - Rising edge trigger enable of line 14"]
pub type RTEN14_R = crate::BitReader<bool>;
#[doc = "Field `RTEN14` writer - Rising edge trigger enable of line 14"]
pub type RTEN14_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTEN_SPEC, bool, O>;
#[doc = "Field `RTEN15` reader - Rising edge trigger enable of line 15"]
pub type RTEN15_R = crate::BitReader<bool>;
#[doc = "Field `RTEN15` writer - Rising edge trigger enable of line 15"]
pub type RTEN15_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTEN_SPEC, bool, O>;
#[doc = "Field `RTEN16` reader - Rising edge trigger enable of line 16"]
pub type RTEN16_R = crate::BitReader<bool>;
#[doc = "Field `RTEN16` writer - Rising edge trigger enable of line 16"]
pub type RTEN16_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTEN_SPEC, bool, O>;
#[doc = "Field `RTEN17` reader - Rising edge trigger enable of line 17"]
pub type RTEN17_R = crate::BitReader<bool>;
#[doc = "Field `RTEN17` writer - Rising edge trigger enable of line 17"]
pub type RTEN17_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTEN_SPEC, bool, O>;
#[doc = "Field `RTEN18` reader - Rising edge trigger enable of line 18"]
pub type RTEN18_R = crate::BitReader<bool>;
#[doc = "Field `RTEN18` writer - Rising edge trigger enable of line 18"]
pub type RTEN18_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Rising edge trigger enable of line 0"]
    #[inline(always)]
    pub fn rten0(&self) -> RTEN0_R {
        RTEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rising edge trigger enable of line 1"]
    #[inline(always)]
    pub fn rten1(&self) -> RTEN1_R {
        RTEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rising edge trigger enable of line 2"]
    #[inline(always)]
    pub fn rten2(&self) -> RTEN2_R {
        RTEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rising edge trigger enable of line 3"]
    #[inline(always)]
    pub fn rten3(&self) -> RTEN3_R {
        RTEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rising edge trigger enable of line 4"]
    #[inline(always)]
    pub fn rten4(&self) -> RTEN4_R {
        RTEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rising edge trigger enable of line 5"]
    #[inline(always)]
    pub fn rten5(&self) -> RTEN5_R {
        RTEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rising edge trigger enable of line 6"]
    #[inline(always)]
    pub fn rten6(&self) -> RTEN6_R {
        RTEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rising edge trigger enable of line 7"]
    #[inline(always)]
    pub fn rten7(&self) -> RTEN7_R {
        RTEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Rising edge trigger enable of line 8"]
    #[inline(always)]
    pub fn rten8(&self) -> RTEN8_R {
        RTEN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Rising edge trigger enable of line 9"]
    #[inline(always)]
    pub fn rten9(&self) -> RTEN9_R {
        RTEN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Rising edge trigger enable of line 10"]
    #[inline(always)]
    pub fn rten10(&self) -> RTEN10_R {
        RTEN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Rising edge trigger enable of line 11"]
    #[inline(always)]
    pub fn rten11(&self) -> RTEN11_R {
        RTEN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Rising edge trigger enable of line 12"]
    #[inline(always)]
    pub fn rten12(&self) -> RTEN12_R {
        RTEN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Rising edge trigger enable of line 13"]
    #[inline(always)]
    pub fn rten13(&self) -> RTEN13_R {
        RTEN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Rising edge trigger enable of line 14"]
    #[inline(always)]
    pub fn rten14(&self) -> RTEN14_R {
        RTEN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rising edge trigger enable of line 15"]
    #[inline(always)]
    pub fn rten15(&self) -> RTEN15_R {
        RTEN15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Rising edge trigger enable of line 16"]
    #[inline(always)]
    pub fn rten16(&self) -> RTEN16_R {
        RTEN16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Rising edge trigger enable of line 17"]
    #[inline(always)]
    pub fn rten17(&self) -> RTEN17_R {
        RTEN17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Rising edge trigger enable of line 18"]
    #[inline(always)]
    pub fn rten18(&self) -> RTEN18_R {
        RTEN18_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rising edge trigger enable of line 0"]
    #[inline(always)]
    #[must_use]
    pub fn rten0(&mut self) -> RTEN0_W<0> {
        RTEN0_W::new(self)
    }
    #[doc = "Bit 1 - Rising edge trigger enable of line 1"]
    #[inline(always)]
    #[must_use]
    pub fn rten1(&mut self) -> RTEN1_W<1> {
        RTEN1_W::new(self)
    }
    #[doc = "Bit 2 - Rising edge trigger enable of line 2"]
    #[inline(always)]
    #[must_use]
    pub fn rten2(&mut self) -> RTEN2_W<2> {
        RTEN2_W::new(self)
    }
    #[doc = "Bit 3 - Rising edge trigger enable of line 3"]
    #[inline(always)]
    #[must_use]
    pub fn rten3(&mut self) -> RTEN3_W<3> {
        RTEN3_W::new(self)
    }
    #[doc = "Bit 4 - Rising edge trigger enable of line 4"]
    #[inline(always)]
    #[must_use]
    pub fn rten4(&mut self) -> RTEN4_W<4> {
        RTEN4_W::new(self)
    }
    #[doc = "Bit 5 - Rising edge trigger enable of line 5"]
    #[inline(always)]
    #[must_use]
    pub fn rten5(&mut self) -> RTEN5_W<5> {
        RTEN5_W::new(self)
    }
    #[doc = "Bit 6 - Rising edge trigger enable of line 6"]
    #[inline(always)]
    #[must_use]
    pub fn rten6(&mut self) -> RTEN6_W<6> {
        RTEN6_W::new(self)
    }
    #[doc = "Bit 7 - Rising edge trigger enable of line 7"]
    #[inline(always)]
    #[must_use]
    pub fn rten7(&mut self) -> RTEN7_W<7> {
        RTEN7_W::new(self)
    }
    #[doc = "Bit 8 - Rising edge trigger enable of line 8"]
    #[inline(always)]
    #[must_use]
    pub fn rten8(&mut self) -> RTEN8_W<8> {
        RTEN8_W::new(self)
    }
    #[doc = "Bit 9 - Rising edge trigger enable of line 9"]
    #[inline(always)]
    #[must_use]
    pub fn rten9(&mut self) -> RTEN9_W<9> {
        RTEN9_W::new(self)
    }
    #[doc = "Bit 10 - Rising edge trigger enable of line 10"]
    #[inline(always)]
    #[must_use]
    pub fn rten10(&mut self) -> RTEN10_W<10> {
        RTEN10_W::new(self)
    }
    #[doc = "Bit 11 - Rising edge trigger enable of line 11"]
    #[inline(always)]
    #[must_use]
    pub fn rten11(&mut self) -> RTEN11_W<11> {
        RTEN11_W::new(self)
    }
    #[doc = "Bit 12 - Rising edge trigger enable of line 12"]
    #[inline(always)]
    #[must_use]
    pub fn rten12(&mut self) -> RTEN12_W<12> {
        RTEN12_W::new(self)
    }
    #[doc = "Bit 13 - Rising edge trigger enable of line 13"]
    #[inline(always)]
    #[must_use]
    pub fn rten13(&mut self) -> RTEN13_W<13> {
        RTEN13_W::new(self)
    }
    #[doc = "Bit 14 - Rising edge trigger enable of line 14"]
    #[inline(always)]
    #[must_use]
    pub fn rten14(&mut self) -> RTEN14_W<14> {
        RTEN14_W::new(self)
    }
    #[doc = "Bit 15 - Rising edge trigger enable of line 15"]
    #[inline(always)]
    #[must_use]
    pub fn rten15(&mut self) -> RTEN15_W<15> {
        RTEN15_W::new(self)
    }
    #[doc = "Bit 16 - Rising edge trigger enable of line 16"]
    #[inline(always)]
    #[must_use]
    pub fn rten16(&mut self) -> RTEN16_W<16> {
        RTEN16_W::new(self)
    }
    #[doc = "Bit 17 - Rising edge trigger enable of line 17"]
    #[inline(always)]
    #[must_use]
    pub fn rten17(&mut self) -> RTEN17_W<17> {
        RTEN17_W::new(self)
    }
    #[doc = "Bit 18 - Rising edge trigger enable of line 18"]
    #[inline(always)]
    #[must_use]
    pub fn rten18(&mut self) -> RTEN18_W<18> {
        RTEN18_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Rising Edge Trigger Enable register (EXTI_RTEN)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rten](index.html) module"]
pub struct RTEN_SPEC;
impl crate::RegisterSpec for RTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rten::R](R) reader structure"]
impl crate::Readable for RTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rten::W](W) writer structure"]
impl crate::Writable for RTEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTEN to value 0"]
impl crate::Resettable for RTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
