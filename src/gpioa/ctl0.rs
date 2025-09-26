#[doc = "Register `CTL0` reader"]
pub type R = crate::R<CTL0_SPEC>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<CTL0_SPEC>;
#[doc = "Field `MD0` reader - Port x mode bits (x = 0)"]
pub type MD0_R = crate::FieldReader;
#[doc = "Field `MD0` writer - Port x mode bits (x = 0)"]
pub type MD0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CTL0` reader - Port x configuration bits (x = 0)"]
pub type CTL0_R = crate::FieldReader;
#[doc = "Field `CTL0` writer - Port x configuration bits (x = 0)"]
pub type CTL0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `MD1` reader - Port x mode bits (x = 1)"]
pub type MD1_R = crate::FieldReader;
#[doc = "Field `MD1` writer - Port x mode bits (x = 1)"]
pub type MD1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CTL1` reader - Port x configuration bits (x = 1)"]
pub type CTL1_R = crate::FieldReader;
#[doc = "Field `CTL1` writer - Port x configuration bits (x = 1)"]
pub type CTL1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `MD2` reader - Port x mode bits (x = 2 )"]
pub type MD2_R = crate::FieldReader;
#[doc = "Field `MD2` writer - Port x mode bits (x = 2 )"]
pub type MD2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CTL2` reader - Port x configuration bits (x = 2)"]
pub type CTL2_R = crate::FieldReader;
#[doc = "Field `CTL2` writer - Port x configuration bits (x = 2)"]
pub type CTL2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `MD3` reader - Port x mode bits (x = 3 )"]
pub type MD3_R = crate::FieldReader;
#[doc = "Field `MD3` writer - Port x mode bits (x = 3 )"]
pub type MD3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CTL3` reader - Port x configuration bits (x = 3)"]
pub type CTL3_R = crate::FieldReader;
#[doc = "Field `CTL3` writer - Port x configuration bits (x = 3)"]
pub type CTL3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `MD4` reader - Port x mode bits (x = 4)"]
pub type MD4_R = crate::FieldReader;
#[doc = "Field `MD4` writer - Port x mode bits (x = 4)"]
pub type MD4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CTL4` reader - Port x configuration bits (x = 4)"]
pub type CTL4_R = crate::FieldReader;
#[doc = "Field `CTL4` writer - Port x configuration bits (x = 4)"]
pub type CTL4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `MD5` reader - Port x mode bits (x = 5)"]
pub type MD5_R = crate::FieldReader;
#[doc = "Field `MD5` writer - Port x mode bits (x = 5)"]
pub type MD5_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CTL5` reader - Port x configuration bits (x = 5)"]
pub type CTL5_R = crate::FieldReader;
#[doc = "Field `CTL5` writer - Port x configuration bits (x = 5)"]
pub type CTL5_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `MD6` reader - Port x mode bits (x = 6)"]
pub type MD6_R = crate::FieldReader;
#[doc = "Field `MD6` writer - Port x mode bits (x = 6)"]
pub type MD6_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CTL6` reader - Port x configuration bits (x = 6)"]
pub type CTL6_R = crate::FieldReader;
#[doc = "Field `CTL6` writer - Port x configuration bits (x = 6)"]
pub type CTL6_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `MD7` reader - Port x mode bits (x = 7)"]
pub type MD7_R = crate::FieldReader;
#[doc = "Field `MD7` writer - Port x mode bits (x = 7)"]
pub type MD7_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CTL7` reader - Port x configuration bits (x = 7)"]
pub type CTL7_R = crate::FieldReader;
#[doc = "Field `CTL7` writer - Port x configuration bits (x = 7)"]
pub type CTL7_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Port x mode bits (x = 0)"]
    #[inline(always)]
    pub fn md0(&self) -> MD0_R {
        MD0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (x = 0)"]
    #[inline(always)]
    pub fn ctl0(&self) -> CTL0_R {
        CTL0_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port x mode bits (x = 1)"]
    #[inline(always)]
    pub fn md1(&self) -> MD1_R {
        MD1_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (x = 1)"]
    #[inline(always)]
    pub fn ctl1(&self) -> CTL1_R {
        CTL1_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port x mode bits (x = 2 )"]
    #[inline(always)]
    pub fn md2(&self) -> MD2_R {
        MD2_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (x = 2)"]
    #[inline(always)]
    pub fn ctl2(&self) -> CTL2_R {
        CTL2_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port x mode bits (x = 3 )"]
    #[inline(always)]
    pub fn md3(&self) -> MD3_R {
        MD3_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (x = 3)"]
    #[inline(always)]
    pub fn ctl3(&self) -> CTL3_R {
        CTL3_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port x mode bits (x = 4)"]
    #[inline(always)]
    pub fn md4(&self) -> MD4_R {
        MD4_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port x configuration bits (x = 4)"]
    #[inline(always)]
    pub fn ctl4(&self) -> CTL4_R {
        CTL4_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port x mode bits (x = 5)"]
    #[inline(always)]
    pub fn md5(&self) -> MD5_R {
        MD5_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port x configuration bits (x = 5)"]
    #[inline(always)]
    pub fn ctl5(&self) -> CTL5_R {
        CTL5_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port x mode bits (x = 6)"]
    #[inline(always)]
    pub fn md6(&self) -> MD6_R {
        MD6_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port x configuration bits (x = 6)"]
    #[inline(always)]
    pub fn ctl6(&self) -> CTL6_R {
        CTL6_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port x mode bits (x = 7)"]
    #[inline(always)]
    pub fn md7(&self) -> MD7_R {
        MD7_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port x configuration bits (x = 7)"]
    #[inline(always)]
    pub fn ctl7(&self) -> CTL7_R {
        CTL7_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port x mode bits (x = 0)"]
    #[inline(always)]
    #[must_use]
    pub fn md0(&mut self) -> MD0_W<CTL0_SPEC, 0> {
        MD0_W::new(self)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (x = 0)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl0(&mut self) -> CTL0_W<CTL0_SPEC, 2> {
        CTL0_W::new(self)
    }
    #[doc = "Bits 4:5 - Port x mode bits (x = 1)"]
    #[inline(always)]
    #[must_use]
    pub fn md1(&mut self) -> MD1_W<CTL0_SPEC, 4> {
        MD1_W::new(self)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (x = 1)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl1(&mut self) -> CTL1_W<CTL0_SPEC, 6> {
        CTL1_W::new(self)
    }
    #[doc = "Bits 8:9 - Port x mode bits (x = 2 )"]
    #[inline(always)]
    #[must_use]
    pub fn md2(&mut self) -> MD2_W<CTL0_SPEC, 8> {
        MD2_W::new(self)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (x = 2)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl2(&mut self) -> CTL2_W<CTL0_SPEC, 10> {
        CTL2_W::new(self)
    }
    #[doc = "Bits 12:13 - Port x mode bits (x = 3 )"]
    #[inline(always)]
    #[must_use]
    pub fn md3(&mut self) -> MD3_W<CTL0_SPEC, 12> {
        MD3_W::new(self)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (x = 3)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl3(&mut self) -> CTL3_W<CTL0_SPEC, 14> {
        CTL3_W::new(self)
    }
    #[doc = "Bits 16:17 - Port x mode bits (x = 4)"]
    #[inline(always)]
    #[must_use]
    pub fn md4(&mut self) -> MD4_W<CTL0_SPEC, 16> {
        MD4_W::new(self)
    }
    #[doc = "Bits 18:19 - Port x configuration bits (x = 4)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl4(&mut self) -> CTL4_W<CTL0_SPEC, 18> {
        CTL4_W::new(self)
    }
    #[doc = "Bits 20:21 - Port x mode bits (x = 5)"]
    #[inline(always)]
    #[must_use]
    pub fn md5(&mut self) -> MD5_W<CTL0_SPEC, 20> {
        MD5_W::new(self)
    }
    #[doc = "Bits 22:23 - Port x configuration bits (x = 5)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl5(&mut self) -> CTL5_W<CTL0_SPEC, 22> {
        CTL5_W::new(self)
    }
    #[doc = "Bits 24:25 - Port x mode bits (x = 6)"]
    #[inline(always)]
    #[must_use]
    pub fn md6(&mut self) -> MD6_W<CTL0_SPEC, 24> {
        MD6_W::new(self)
    }
    #[doc = "Bits 26:27 - Port x configuration bits (x = 6)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl6(&mut self) -> CTL6_W<CTL0_SPEC, 26> {
        CTL6_W::new(self)
    }
    #[doc = "Bits 28:29 - Port x mode bits (x = 7)"]
    #[inline(always)]
    #[must_use]
    pub fn md7(&mut self) -> MD7_W<CTL0_SPEC, 28> {
        MD7_W::new(self)
    }
    #[doc = "Bits 30:31 - Port x configuration bits (x = 7)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl7(&mut self) -> CTL7_W<CTL0_SPEC, 30> {
        CTL7_W::new(self)
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
#[doc = "port control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL0_SPEC;
impl crate::RegisterSpec for CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl0::R`](R) reader structure"]
impl crate::Readable for CTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl0::W`](W) writer structure"]
impl crate::Writable for CTL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL0 to value 0x4444_4444"]
impl crate::Resettable for CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0x4444_4444;
}
