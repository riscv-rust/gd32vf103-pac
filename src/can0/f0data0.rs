#[doc = "Register `F0DATA0` reader"]
pub struct R(crate::R<F0DATA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<F0DATA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<F0DATA0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<F0DATA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `F0DATA0` writer"]
pub struct W(crate::W<F0DATA0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<F0DATA0_SPEC>;
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
impl From<crate::W<F0DATA0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<F0DATA0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FD0` reader - Filter bits"]
pub type FD0_R = crate::BitReader<bool>;
#[doc = "Field `FD0` writer - Filter bits"]
pub type FD0_W<'a, const O: u8> = crate::BitWriter<'a, u32, F0DATA0_SPEC, bool, O>;
#[doc = "Field `FD1` reader - Filter bits"]
pub type FD1_R = crate::BitReader<bool>;
#[doc = "Field `FD1` writer - Filter bits"]
pub type FD1_W<'a, const O: u8> = crate::BitWriter<'a, u32, F0DATA0_SPEC, bool, O>;
#[doc = "Field `FD2` reader - Filter bits"]
pub type FD2_R = crate::BitReader<bool>;
#[doc = "Field `FD2` writer - Filter bits"]
pub type FD2_W<'a, const O: u8> = crate::BitWriter<'a, u32, F0DATA0_SPEC, bool, O>;
#[doc = "Field `FD3` reader - Filter bits"]
pub type FD3_R = crate::BitReader<bool>;
#[doc = "Field `FD3` writer - Filter bits"]
pub type FD3_W<'a, const O: u8> = crate::BitWriter<'a, u32, F0DATA0_SPEC, bool, O>;
#[doc = "Field `FD4` reader - Filter bits"]
pub type FD4_R = crate::BitReader<bool>;
#[doc = "Field `FD4` writer - Filter bits"]
pub type FD4_W<'a, const O: u8> = crate::BitWriter<'a, u32, F0DATA0_SPEC, bool, O>;
#[doc = "Field `FD5` reader - Filter bits"]
pub type FD5_R = crate::BitReader<bool>;
#[doc = "Field `FD5` writer - Filter bits"]
pub type FD5_W<'a, const O: u8> = crate::BitWriter<'a, u32, F0DATA0_SPEC, bool, O>;
#[doc = "Field `FD6` reader - Filter bits"]
pub type FD6_R = crate::BitReader<bool>;
#[doc = "Field `FD6` writer - Filter bits"]
pub type FD6_W<'a, const O: u8> = crate::BitWriter<'a, u32, F0DATA0_SPEC, bool, O>;
#[doc = "Field `FD7` reader - Filter bits"]
pub type FD7_R = crate::BitReader<bool>;
#[doc = "Field `FD7` writer - Filter bits"]
pub type FD7_W<'a, const O: u8> = crate::BitWriter<'a, u32, F0DATA0_SPEC, bool, O>;
#[doc = "Field `FD8` reader - Filter bits"]
pub type FD8_R = crate::BitReader<bool>;
#[doc = "Field `FD8` writer - Filter bits"]
pub type FD8_W<'a, const O: u8> = crate::BitWriter<'a, u32, F0DATA0_SPEC, bool, O>;
#[doc = "Field `FD9` reader - Filter bits"]
pub type FD9_R = crate::BitReader<bool>;
#[doc = "Field `FD9` writer - Filter bits"]
pub type FD9_W<'a, const O: u8> = crate::BitWriter<'a, u32, F0DATA0_SPEC, bool, O>;
#[doc = "Field `FD10` reader - Filter bits"]
pub type FD10_R = crate::BitReader<bool>;
#[doc = "Field `FD10` writer - Filter bits"]
pub type FD10_W<'a, const O: u8> = crate::BitWriter<'a, u32, F0DATA0_SPEC, bool, O>;
#[doc = "Field `FD11` reader - Filter bits"]
pub type FD11_R = crate::BitReader<bool>;
#[doc = "Field `FD11` writer - Filter bits"]
pub type FD11_W<'a, const O: u8> = crate::BitWriter<'a, u32, F0DATA0_SPEC, bool, O>;
#[doc = "Field `FD12` reader - Filter bits"]
pub type FD12_R = crate::BitReader<bool>;
#[doc = "Field `FD12` writer - Filter bits"]
pub type FD12_W<'a, const O: u8> = crate::BitWriter<'a, u32, F0DATA0_SPEC, bool, O>;
#[doc = "Field `FD13` reader - Filter bits"]
pub type FD13_R = crate::BitReader<bool>;
#[doc = "Field `FD13` writer - Filter bits"]
pub type FD13_W<'a, const O: u8> = crate::BitWriter<'a, u32, F0DATA0_SPEC, bool, O>;
#[doc = "Field `FD14` reader - Filter bits"]
pub type FD14_R = crate::BitReader<bool>;
#[doc = "Field `FD14` writer - Filter bits"]
pub type FD14_W<'a, const O: u8> = crate::BitWriter<'a, u32, F0DATA0_SPEC, bool, O>;
#[doc = "Field `FD15` reader - Filter bits"]
pub type FD15_R = crate::BitReader<bool>;
#[doc = "Field `FD15` writer - Filter bits"]
pub type FD15_W<'a, const O: u8> = crate::BitWriter<'a, u32, F0DATA0_SPEC, bool, O>;
#[doc = "Field `FD16` reader - Filter bits"]
pub type FD16_R = crate::BitReader<bool>;
#[doc = "Field `FD16` writer - Filter bits"]
pub type FD16_W<'a, const O: u8> = crate::BitWriter<'a, u32, F0DATA0_SPEC, bool, O>;
#[doc = "Field `FD17` reader - Filter bits"]
pub type FD17_R = crate::BitReader<bool>;
#[doc = "Field `FD17` writer - Filter bits"]
pub type FD17_W<'a, const O: u8> = crate::BitWriter<'a, u32, F0DATA0_SPEC, bool, O>;
#[doc = "Field `FD18` reader - Filter bits"]
pub type FD18_R = crate::BitReader<bool>;
#[doc = "Field `FD18` writer - Filter bits"]
pub type FD18_W<'a, const O: u8> = crate::BitWriter<'a, u32, F0DATA0_SPEC, bool, O>;
#[doc = "Field `FD19` reader - Filter bits"]
pub type FD19_R = crate::BitReader<bool>;
#[doc = "Field `FD19` writer - Filter bits"]
pub type FD19_W<'a, const O: u8> = crate::BitWriter<'a, u32, F0DATA0_SPEC, bool, O>;
#[doc = "Field `FD20` reader - Filter bits"]
pub type FD20_R = crate::BitReader<bool>;
#[doc = "Field `FD20` writer - Filter bits"]
pub type FD20_W<'a, const O: u8> = crate::BitWriter<'a, u32, F0DATA0_SPEC, bool, O>;
#[doc = "Field `FD21` reader - Filter bits"]
pub type FD21_R = crate::BitReader<bool>;
#[doc = "Field `FD21` writer - Filter bits"]
pub type FD21_W<'a, const O: u8> = crate::BitWriter<'a, u32, F0DATA0_SPEC, bool, O>;
#[doc = "Field `FD22` reader - Filter bits"]
pub type FD22_R = crate::BitReader<bool>;
#[doc = "Field `FD22` writer - Filter bits"]
pub type FD22_W<'a, const O: u8> = crate::BitWriter<'a, u32, F0DATA0_SPEC, bool, O>;
#[doc = "Field `FD23` reader - Filter bits"]
pub type FD23_R = crate::BitReader<bool>;
#[doc = "Field `FD23` writer - Filter bits"]
pub type FD23_W<'a, const O: u8> = crate::BitWriter<'a, u32, F0DATA0_SPEC, bool, O>;
#[doc = "Field `FD24` reader - Filter bits"]
pub type FD24_R = crate::BitReader<bool>;
#[doc = "Field `FD24` writer - Filter bits"]
pub type FD24_W<'a, const O: u8> = crate::BitWriter<'a, u32, F0DATA0_SPEC, bool, O>;
#[doc = "Field `FD25` reader - Filter bits"]
pub type FD25_R = crate::BitReader<bool>;
#[doc = "Field `FD25` writer - Filter bits"]
pub type FD25_W<'a, const O: u8> = crate::BitWriter<'a, u32, F0DATA0_SPEC, bool, O>;
#[doc = "Field `FD26` reader - Filter bits"]
pub type FD26_R = crate::BitReader<bool>;
#[doc = "Field `FD26` writer - Filter bits"]
pub type FD26_W<'a, const O: u8> = crate::BitWriter<'a, u32, F0DATA0_SPEC, bool, O>;
#[doc = "Field `FD27` reader - Filter bits"]
pub type FD27_R = crate::BitReader<bool>;
#[doc = "Field `FD27` writer - Filter bits"]
pub type FD27_W<'a, const O: u8> = crate::BitWriter<'a, u32, F0DATA0_SPEC, bool, O>;
#[doc = "Field `FD28` reader - Filter bits"]
pub type FD28_R = crate::BitReader<bool>;
#[doc = "Field `FD28` writer - Filter bits"]
pub type FD28_W<'a, const O: u8> = crate::BitWriter<'a, u32, F0DATA0_SPEC, bool, O>;
#[doc = "Field `FD29` reader - Filter bits"]
pub type FD29_R = crate::BitReader<bool>;
#[doc = "Field `FD29` writer - Filter bits"]
pub type FD29_W<'a, const O: u8> = crate::BitWriter<'a, u32, F0DATA0_SPEC, bool, O>;
#[doc = "Field `FD30` reader - Filter bits"]
pub type FD30_R = crate::BitReader<bool>;
#[doc = "Field `FD30` writer - Filter bits"]
pub type FD30_W<'a, const O: u8> = crate::BitWriter<'a, u32, F0DATA0_SPEC, bool, O>;
#[doc = "Field `FD31` reader - Filter bits"]
pub type FD31_R = crate::BitReader<bool>;
#[doc = "Field `FD31` writer - Filter bits"]
pub type FD31_W<'a, const O: u8> = crate::BitWriter<'a, u32, F0DATA0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Filter bits"]
    #[inline(always)]
    pub fn fd0(&self) -> FD0_R {
        FD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Filter bits"]
    #[inline(always)]
    pub fn fd1(&self) -> FD1_R {
        FD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Filter bits"]
    #[inline(always)]
    pub fn fd2(&self) -> FD2_R {
        FD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Filter bits"]
    #[inline(always)]
    pub fn fd3(&self) -> FD3_R {
        FD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter bits"]
    #[inline(always)]
    pub fn fd4(&self) -> FD4_R {
        FD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter bits"]
    #[inline(always)]
    pub fn fd5(&self) -> FD5_R {
        FD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Filter bits"]
    #[inline(always)]
    pub fn fd6(&self) -> FD6_R {
        FD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Filter bits"]
    #[inline(always)]
    pub fn fd7(&self) -> FD7_R {
        FD7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter bits"]
    #[inline(always)]
    pub fn fd8(&self) -> FD8_R {
        FD8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter bits"]
    #[inline(always)]
    pub fn fd9(&self) -> FD9_R {
        FD9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Filter bits"]
    #[inline(always)]
    pub fn fd10(&self) -> FD10_R {
        FD10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Filter bits"]
    #[inline(always)]
    pub fn fd11(&self) -> FD11_R {
        FD11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter bits"]
    #[inline(always)]
    pub fn fd12(&self) -> FD12_R {
        FD12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Filter bits"]
    #[inline(always)]
    pub fn fd13(&self) -> FD13_R {
        FD13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Filter bits"]
    #[inline(always)]
    pub fn fd14(&self) -> FD14_R {
        FD14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Filter bits"]
    #[inline(always)]
    pub fn fd15(&self) -> FD15_R {
        FD15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Filter bits"]
    #[inline(always)]
    pub fn fd16(&self) -> FD16_R {
        FD16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Filter bits"]
    #[inline(always)]
    pub fn fd17(&self) -> FD17_R {
        FD17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Filter bits"]
    #[inline(always)]
    pub fn fd18(&self) -> FD18_R {
        FD18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Filter bits"]
    #[inline(always)]
    pub fn fd19(&self) -> FD19_R {
        FD19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Filter bits"]
    #[inline(always)]
    pub fn fd20(&self) -> FD20_R {
        FD20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Filter bits"]
    #[inline(always)]
    pub fn fd21(&self) -> FD21_R {
        FD21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Filter bits"]
    #[inline(always)]
    pub fn fd22(&self) -> FD22_R {
        FD22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Filter bits"]
    #[inline(always)]
    pub fn fd23(&self) -> FD23_R {
        FD23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Filter bits"]
    #[inline(always)]
    pub fn fd24(&self) -> FD24_R {
        FD24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Filter bits"]
    #[inline(always)]
    pub fn fd25(&self) -> FD25_R {
        FD25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Filter bits"]
    #[inline(always)]
    pub fn fd26(&self) -> FD26_R {
        FD26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Filter bits"]
    #[inline(always)]
    pub fn fd27(&self) -> FD27_R {
        FD27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Filter bits"]
    #[inline(always)]
    pub fn fd28(&self) -> FD28_R {
        FD28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Filter bits"]
    #[inline(always)]
    pub fn fd29(&self) -> FD29_R {
        FD29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Filter bits"]
    #[inline(always)]
    pub fn fd30(&self) -> FD30_R {
        FD30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Filter bits"]
    #[inline(always)]
    pub fn fd31(&self) -> FD31_R {
        FD31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filter bits"]
    #[inline(always)]
    #[must_use]
    pub fn fd0(&mut self) -> FD0_W<0> {
        FD0_W::new(self)
    }
    #[doc = "Bit 1 - Filter bits"]
    #[inline(always)]
    #[must_use]
    pub fn fd1(&mut self) -> FD1_W<1> {
        FD1_W::new(self)
    }
    #[doc = "Bit 2 - Filter bits"]
    #[inline(always)]
    #[must_use]
    pub fn fd2(&mut self) -> FD2_W<2> {
        FD2_W::new(self)
    }
    #[doc = "Bit 3 - Filter bits"]
    #[inline(always)]
    #[must_use]
    pub fn fd3(&mut self) -> FD3_W<3> {
        FD3_W::new(self)
    }
    #[doc = "Bit 4 - Filter bits"]
    #[inline(always)]
    #[must_use]
    pub fn fd4(&mut self) -> FD4_W<4> {
        FD4_W::new(self)
    }
    #[doc = "Bit 5 - Filter bits"]
    #[inline(always)]
    #[must_use]
    pub fn fd5(&mut self) -> FD5_W<5> {
        FD5_W::new(self)
    }
    #[doc = "Bit 6 - Filter bits"]
    #[inline(always)]
    #[must_use]
    pub fn fd6(&mut self) -> FD6_W<6> {
        FD6_W::new(self)
    }
    #[doc = "Bit 7 - Filter bits"]
    #[inline(always)]
    #[must_use]
    pub fn fd7(&mut self) -> FD7_W<7> {
        FD7_W::new(self)
    }
    #[doc = "Bit 8 - Filter bits"]
    #[inline(always)]
    #[must_use]
    pub fn fd8(&mut self) -> FD8_W<8> {
        FD8_W::new(self)
    }
    #[doc = "Bit 9 - Filter bits"]
    #[inline(always)]
    #[must_use]
    pub fn fd9(&mut self) -> FD9_W<9> {
        FD9_W::new(self)
    }
    #[doc = "Bit 10 - Filter bits"]
    #[inline(always)]
    #[must_use]
    pub fn fd10(&mut self) -> FD10_W<10> {
        FD10_W::new(self)
    }
    #[doc = "Bit 11 - Filter bits"]
    #[inline(always)]
    #[must_use]
    pub fn fd11(&mut self) -> FD11_W<11> {
        FD11_W::new(self)
    }
    #[doc = "Bit 12 - Filter bits"]
    #[inline(always)]
    #[must_use]
    pub fn fd12(&mut self) -> FD12_W<12> {
        FD12_W::new(self)
    }
    #[doc = "Bit 13 - Filter bits"]
    #[inline(always)]
    #[must_use]
    pub fn fd13(&mut self) -> FD13_W<13> {
        FD13_W::new(self)
    }
    #[doc = "Bit 14 - Filter bits"]
    #[inline(always)]
    #[must_use]
    pub fn fd14(&mut self) -> FD14_W<14> {
        FD14_W::new(self)
    }
    #[doc = "Bit 15 - Filter bits"]
    #[inline(always)]
    #[must_use]
    pub fn fd15(&mut self) -> FD15_W<15> {
        FD15_W::new(self)
    }
    #[doc = "Bit 16 - Filter bits"]
    #[inline(always)]
    #[must_use]
    pub fn fd16(&mut self) -> FD16_W<16> {
        FD16_W::new(self)
    }
    #[doc = "Bit 17 - Filter bits"]
    #[inline(always)]
    #[must_use]
    pub fn fd17(&mut self) -> FD17_W<17> {
        FD17_W::new(self)
    }
    #[doc = "Bit 18 - Filter bits"]
    #[inline(always)]
    #[must_use]
    pub fn fd18(&mut self) -> FD18_W<18> {
        FD18_W::new(self)
    }
    #[doc = "Bit 19 - Filter bits"]
    #[inline(always)]
    #[must_use]
    pub fn fd19(&mut self) -> FD19_W<19> {
        FD19_W::new(self)
    }
    #[doc = "Bit 20 - Filter bits"]
    #[inline(always)]
    #[must_use]
    pub fn fd20(&mut self) -> FD20_W<20> {
        FD20_W::new(self)
    }
    #[doc = "Bit 21 - Filter bits"]
    #[inline(always)]
    #[must_use]
    pub fn fd21(&mut self) -> FD21_W<21> {
        FD21_W::new(self)
    }
    #[doc = "Bit 22 - Filter bits"]
    #[inline(always)]
    #[must_use]
    pub fn fd22(&mut self) -> FD22_W<22> {
        FD22_W::new(self)
    }
    #[doc = "Bit 23 - Filter bits"]
    #[inline(always)]
    #[must_use]
    pub fn fd23(&mut self) -> FD23_W<23> {
        FD23_W::new(self)
    }
    #[doc = "Bit 24 - Filter bits"]
    #[inline(always)]
    #[must_use]
    pub fn fd24(&mut self) -> FD24_W<24> {
        FD24_W::new(self)
    }
    #[doc = "Bit 25 - Filter bits"]
    #[inline(always)]
    #[must_use]
    pub fn fd25(&mut self) -> FD25_W<25> {
        FD25_W::new(self)
    }
    #[doc = "Bit 26 - Filter bits"]
    #[inline(always)]
    #[must_use]
    pub fn fd26(&mut self) -> FD26_W<26> {
        FD26_W::new(self)
    }
    #[doc = "Bit 27 - Filter bits"]
    #[inline(always)]
    #[must_use]
    pub fn fd27(&mut self) -> FD27_W<27> {
        FD27_W::new(self)
    }
    #[doc = "Bit 28 - Filter bits"]
    #[inline(always)]
    #[must_use]
    pub fn fd28(&mut self) -> FD28_W<28> {
        FD28_W::new(self)
    }
    #[doc = "Bit 29 - Filter bits"]
    #[inline(always)]
    #[must_use]
    pub fn fd29(&mut self) -> FD29_W<29> {
        FD29_W::new(self)
    }
    #[doc = "Bit 30 - Filter bits"]
    #[inline(always)]
    #[must_use]
    pub fn fd30(&mut self) -> FD30_W<30> {
        FD30_W::new(self)
    }
    #[doc = "Bit 31 - Filter bits"]
    #[inline(always)]
    #[must_use]
    pub fn fd31(&mut self) -> FD31_W<31> {
        FD31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Filter 0 data 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f0data0](index.html) module"]
pub struct F0DATA0_SPEC;
impl crate::RegisterSpec for F0DATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [f0data0::R](R) reader structure"]
impl crate::Readable for F0DATA0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [f0data0::W](W) writer structure"]
impl crate::Writable for F0DATA0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets F0DATA0 to value 0"]
impl crate::Resettable for F0DATA0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
