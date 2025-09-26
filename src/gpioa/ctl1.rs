#[doc = "Register `CTL1` reader"]
pub type R = crate::R<CTL1_SPEC>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<CTL1_SPEC>;
#[doc = "Field `MD8` reader - Port x mode bits (x = 8)"]
pub type MD8_R = crate::FieldReader;
#[doc = "Field `MD8` writer - Port x mode bits (x = 8)"]
pub type MD8_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CTL8` reader - Port x configuration bits (x = 8)"]
pub type CTL8_R = crate::FieldReader;
#[doc = "Field `CTL8` writer - Port x configuration bits (x = 8)"]
pub type CTL8_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `MD9` reader - Port x mode bits (x = 9)"]
pub type MD9_R = crate::FieldReader;
#[doc = "Field `MD9` writer - Port x mode bits (x = 9)"]
pub type MD9_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CTL9` reader - Port x configuration bits (x = 9)"]
pub type CTL9_R = crate::FieldReader;
#[doc = "Field `CTL9` writer - Port x configuration bits (x = 9)"]
pub type CTL9_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `MD10` reader - Port x mode bits (x = 10 )"]
pub type MD10_R = crate::FieldReader;
#[doc = "Field `MD10` writer - Port x mode bits (x = 10 )"]
pub type MD10_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CTL10` reader - Port x configuration bits (x = 10)"]
pub type CTL10_R = crate::FieldReader;
#[doc = "Field `CTL10` writer - Port x configuration bits (x = 10)"]
pub type CTL10_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `MD11` reader - Port x mode bits (x = 11 )"]
pub type MD11_R = crate::FieldReader;
#[doc = "Field `MD11` writer - Port x mode bits (x = 11 )"]
pub type MD11_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CTL11` reader - Port x configuration bits (x = 11)"]
pub type CTL11_R = crate::FieldReader;
#[doc = "Field `CTL11` writer - Port x configuration bits (x = 11)"]
pub type CTL11_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `MD12` reader - Port x mode bits (x = 12)"]
pub type MD12_R = crate::FieldReader;
#[doc = "Field `MD12` writer - Port x mode bits (x = 12)"]
pub type MD12_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CTL12` reader - Port x configuration bits (x = 12)"]
pub type CTL12_R = crate::FieldReader;
#[doc = "Field `CTL12` writer - Port x configuration bits (x = 12)"]
pub type CTL12_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `MD13` reader - Port x mode bits (x = 13)"]
pub type MD13_R = crate::FieldReader;
#[doc = "Field `MD13` writer - Port x mode bits (x = 13)"]
pub type MD13_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CTL13` reader - Port x configuration bits (x = 13)"]
pub type CTL13_R = crate::FieldReader;
#[doc = "Field `CTL13` writer - Port x configuration bits (x = 13)"]
pub type CTL13_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `MD14` reader - Port x mode bits (x = 14)"]
pub type MD14_R = crate::FieldReader;
#[doc = "Field `MD14` writer - Port x mode bits (x = 14)"]
pub type MD14_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CTL14` reader - Port x configuration bits (x = 14)"]
pub type CTL14_R = crate::FieldReader;
#[doc = "Field `CTL14` writer - Port x configuration bits (x = 14)"]
pub type CTL14_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `MD15` reader - Port x mode bits (x = 15)"]
pub type MD15_R = crate::FieldReader;
#[doc = "Field `MD15` writer - Port x mode bits (x = 15)"]
pub type MD15_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CTL15` reader - Port x configuration bits (x = 15)"]
pub type CTL15_R = crate::FieldReader;
#[doc = "Field `CTL15` writer - Port x configuration bits (x = 15)"]
pub type CTL15_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Port x mode bits (x = 8)"]
    #[inline(always)]
    pub fn md8(&self) -> MD8_R {
        MD8_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (x = 8)"]
    #[inline(always)]
    pub fn ctl8(&self) -> CTL8_R {
        CTL8_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port x mode bits (x = 9)"]
    #[inline(always)]
    pub fn md9(&self) -> MD9_R {
        MD9_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (x = 9)"]
    #[inline(always)]
    pub fn ctl9(&self) -> CTL9_R {
        CTL9_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port x mode bits (x = 10 )"]
    #[inline(always)]
    pub fn md10(&self) -> MD10_R {
        MD10_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (x = 10)"]
    #[inline(always)]
    pub fn ctl10(&self) -> CTL10_R {
        CTL10_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port x mode bits (x = 11 )"]
    #[inline(always)]
    pub fn md11(&self) -> MD11_R {
        MD11_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (x = 11)"]
    #[inline(always)]
    pub fn ctl11(&self) -> CTL11_R {
        CTL11_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port x mode bits (x = 12)"]
    #[inline(always)]
    pub fn md12(&self) -> MD12_R {
        MD12_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port x configuration bits (x = 12)"]
    #[inline(always)]
    pub fn ctl12(&self) -> CTL12_R {
        CTL12_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port x mode bits (x = 13)"]
    #[inline(always)]
    pub fn md13(&self) -> MD13_R {
        MD13_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port x configuration bits (x = 13)"]
    #[inline(always)]
    pub fn ctl13(&self) -> CTL13_R {
        CTL13_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port x mode bits (x = 14)"]
    #[inline(always)]
    pub fn md14(&self) -> MD14_R {
        MD14_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port x configuration bits (x = 14)"]
    #[inline(always)]
    pub fn ctl14(&self) -> CTL14_R {
        CTL14_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port x mode bits (x = 15)"]
    #[inline(always)]
    pub fn md15(&self) -> MD15_R {
        MD15_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port x configuration bits (x = 15)"]
    #[inline(always)]
    pub fn ctl15(&self) -> CTL15_R {
        CTL15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port x mode bits (x = 8)"]
    #[inline(always)]
    #[must_use]
    pub fn md8(&mut self) -> MD8_W<CTL1_SPEC, 0> {
        MD8_W::new(self)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (x = 8)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl8(&mut self) -> CTL8_W<CTL1_SPEC, 2> {
        CTL8_W::new(self)
    }
    #[doc = "Bits 4:5 - Port x mode bits (x = 9)"]
    #[inline(always)]
    #[must_use]
    pub fn md9(&mut self) -> MD9_W<CTL1_SPEC, 4> {
        MD9_W::new(self)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (x = 9)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl9(&mut self) -> CTL9_W<CTL1_SPEC, 6> {
        CTL9_W::new(self)
    }
    #[doc = "Bits 8:9 - Port x mode bits (x = 10 )"]
    #[inline(always)]
    #[must_use]
    pub fn md10(&mut self) -> MD10_W<CTL1_SPEC, 8> {
        MD10_W::new(self)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (x = 10)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl10(&mut self) -> CTL10_W<CTL1_SPEC, 10> {
        CTL10_W::new(self)
    }
    #[doc = "Bits 12:13 - Port x mode bits (x = 11 )"]
    #[inline(always)]
    #[must_use]
    pub fn md11(&mut self) -> MD11_W<CTL1_SPEC, 12> {
        MD11_W::new(self)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (x = 11)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl11(&mut self) -> CTL11_W<CTL1_SPEC, 14> {
        CTL11_W::new(self)
    }
    #[doc = "Bits 16:17 - Port x mode bits (x = 12)"]
    #[inline(always)]
    #[must_use]
    pub fn md12(&mut self) -> MD12_W<CTL1_SPEC, 16> {
        MD12_W::new(self)
    }
    #[doc = "Bits 18:19 - Port x configuration bits (x = 12)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl12(&mut self) -> CTL12_W<CTL1_SPEC, 18> {
        CTL12_W::new(self)
    }
    #[doc = "Bits 20:21 - Port x mode bits (x = 13)"]
    #[inline(always)]
    #[must_use]
    pub fn md13(&mut self) -> MD13_W<CTL1_SPEC, 20> {
        MD13_W::new(self)
    }
    #[doc = "Bits 22:23 - Port x configuration bits (x = 13)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl13(&mut self) -> CTL13_W<CTL1_SPEC, 22> {
        CTL13_W::new(self)
    }
    #[doc = "Bits 24:25 - Port x mode bits (x = 14)"]
    #[inline(always)]
    #[must_use]
    pub fn md14(&mut self) -> MD14_W<CTL1_SPEC, 24> {
        MD14_W::new(self)
    }
    #[doc = "Bits 26:27 - Port x configuration bits (x = 14)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl14(&mut self) -> CTL14_W<CTL1_SPEC, 26> {
        CTL14_W::new(self)
    }
    #[doc = "Bits 28:29 - Port x mode bits (x = 15)"]
    #[inline(always)]
    #[must_use]
    pub fn md15(&mut self) -> MD15_W<CTL1_SPEC, 28> {
        MD15_W::new(self)
    }
    #[doc = "Bits 30:31 - Port x configuration bits (x = 15)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl15(&mut self) -> CTL15_W<CTL1_SPEC, 30> {
        CTL15_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "port control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL1_SPEC;
impl crate::RegisterSpec for CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for CTL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for CTL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL1 to value 0x4444_4444"]
impl crate::Resettable for CTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0x4444_4444;
}
