#[doc = "Register `TMI0` reader"]
pub struct R(crate::R<TMI0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMI0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMI0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMI0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMI0` writer"]
pub struct W(crate::W<TMI0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMI0_SPEC>;
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
impl From<crate::W<TMI0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMI0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TEN` reader - Transmit enable"]
pub type TEN_R = crate::BitReader<bool>;
#[doc = "Field `TEN` writer - Transmit enable"]
pub type TEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMI0_SPEC, bool, O>;
#[doc = "Field `FT` reader - Frame type"]
pub type FT_R = crate::BitReader<bool>;
#[doc = "Field `FT` writer - Frame type"]
pub type FT_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMI0_SPEC, bool, O>;
#[doc = "Field `FF` reader - Frame format"]
pub type FF_R = crate::BitReader<bool>;
#[doc = "Field `FF` writer - Frame format"]
pub type FF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMI0_SPEC, bool, O>;
#[doc = "Field `EFID` reader - The frame identifier"]
pub type EFID_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EFID` writer - The frame identifier"]
pub type EFID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TMI0_SPEC, u32, u32, 18, O>;
#[doc = "Field `SFID_EFID` reader - The frame identifier"]
pub type SFID_EFID_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SFID_EFID` writer - The frame identifier"]
pub type SFID_EFID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TMI0_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bit 0 - Transmit enable"]
    #[inline(always)]
    pub fn ten(&self) -> TEN_R {
        TEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Frame type"]
    #[inline(always)]
    pub fn ft(&self) -> FT_R {
        FT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Frame format"]
    #[inline(always)]
    pub fn ff(&self) -> FF_R {
        FF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:20 - The frame identifier"]
    #[inline(always)]
    pub fn efid(&self) -> EFID_R {
        EFID_R::new((self.bits >> 3) & 0x0003_ffff)
    }
    #[doc = "Bits 21:31 - The frame identifier"]
    #[inline(always)]
    pub fn sfid_efid(&self) -> SFID_EFID_R {
        SFID_EFID_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit enable"]
    #[inline(always)]
    #[must_use]
    pub fn ten(&mut self) -> TEN_W<0> {
        TEN_W::new(self)
    }
    #[doc = "Bit 1 - Frame type"]
    #[inline(always)]
    #[must_use]
    pub fn ft(&mut self) -> FT_W<1> {
        FT_W::new(self)
    }
    #[doc = "Bit 2 - Frame format"]
    #[inline(always)]
    #[must_use]
    pub fn ff(&mut self) -> FF_W<2> {
        FF_W::new(self)
    }
    #[doc = "Bits 3:20 - The frame identifier"]
    #[inline(always)]
    #[must_use]
    pub fn efid(&mut self) -> EFID_W<3> {
        EFID_W::new(self)
    }
    #[doc = "Bits 21:31 - The frame identifier"]
    #[inline(always)]
    #[must_use]
    pub fn sfid_efid(&mut self) -> SFID_EFID_W<21> {
        SFID_EFID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit mailbox identifier register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmi0](index.html) module"]
pub struct TMI0_SPEC;
impl crate::RegisterSpec for TMI0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmi0::R](R) reader structure"]
impl crate::Readable for TMI0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmi0::W](W) writer structure"]
impl crate::Writable for TMI0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TMI0 to value 0"]
impl crate::Resettable for TMI0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
