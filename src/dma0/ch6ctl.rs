#[doc = "Register `CH6CTL` reader"]
pub struct R(crate::R<CH6CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH6CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH6CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH6CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH6CTL` writer"]
pub struct W(crate::W<CH6CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH6CTL_SPEC>;
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
impl From<crate::W<CH6CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH6CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHEN` reader - Channel enable"]
pub type CHEN_R = crate::BitReader<bool>;
#[doc = "Field `CHEN` writer - Channel enable"]
pub type CHEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH6CTL_SPEC, bool, O>;
#[doc = "Field `FTFIE` reader - Enable bit for channel full transfer finish interrupt"]
pub type FTFIE_R = crate::BitReader<bool>;
#[doc = "Field `FTFIE` writer - Enable bit for channel full transfer finish interrupt"]
pub type FTFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH6CTL_SPEC, bool, O>;
#[doc = "Field `HTFIE` reader - Enable bit for channel half transfer finish interrupt"]
pub type HTFIE_R = crate::BitReader<bool>;
#[doc = "Field `HTFIE` writer - Enable bit for channel half transfer finish interrupt"]
pub type HTFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH6CTL_SPEC, bool, O>;
#[doc = "Field `ERRIE` reader - Enable bit for channel error interrupt"]
pub type ERRIE_R = crate::BitReader<bool>;
#[doc = "Field `ERRIE` writer - Enable bit for channel error interrupt"]
pub type ERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH6CTL_SPEC, bool, O>;
#[doc = "Field `DIR` reader - Transfer direction"]
pub type DIR_R = crate::BitReader<bool>;
#[doc = "Field `DIR` writer - Transfer direction"]
pub type DIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH6CTL_SPEC, bool, O>;
#[doc = "Field `CMEN` reader - Circular mode enable"]
pub type CMEN_R = crate::BitReader<bool>;
#[doc = "Field `CMEN` writer - Circular mode enable"]
pub type CMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH6CTL_SPEC, bool, O>;
#[doc = "Field `PNAGA` reader - Next address generation algorithm of peripheral"]
pub type PNAGA_R = crate::BitReader<bool>;
#[doc = "Field `PNAGA` writer - Next address generation algorithm of peripheral"]
pub type PNAGA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH6CTL_SPEC, bool, O>;
#[doc = "Field `MNAGA` reader - Next address generation algorithm of memory"]
pub type MNAGA_R = crate::BitReader<bool>;
#[doc = "Field `MNAGA` writer - Next address generation algorithm of memory"]
pub type MNAGA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH6CTL_SPEC, bool, O>;
#[doc = "Field `PWIDTH` reader - Transfer data size of peripheral"]
pub type PWIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWIDTH` writer - Transfer data size of peripheral"]
pub type PWIDTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH6CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `MWIDTH` reader - Transfer data size of memory"]
pub type MWIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MWIDTH` writer - Transfer data size of memory"]
pub type MWIDTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH6CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `PRIO` reader - Priority level"]
pub type PRIO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRIO` writer - Priority level"]
pub type PRIO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH6CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `M2M` reader - Memory to Memory Mode"]
pub type M2M_R = crate::BitReader<bool>;
#[doc = "Field `M2M` writer - Memory to Memory Mode"]
pub type M2M_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH6CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Channel enable"]
    #[inline(always)]
    pub fn chen(&self) -> CHEN_R {
        CHEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable bit for channel full transfer finish interrupt"]
    #[inline(always)]
    pub fn ftfie(&self) -> FTFIE_R {
        FTFIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable bit for channel half transfer finish interrupt"]
    #[inline(always)]
    pub fn htfie(&self) -> HTFIE_R {
        HTFIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable bit for channel error interrupt"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transfer direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Circular mode enable"]
    #[inline(always)]
    pub fn cmen(&self) -> CMEN_R {
        CMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Next address generation algorithm of peripheral"]
    #[inline(always)]
    pub fn pnaga(&self) -> PNAGA_R {
        PNAGA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Next address generation algorithm of memory"]
    #[inline(always)]
    pub fn mnaga(&self) -> MNAGA_R {
        MNAGA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Transfer data size of peripheral"]
    #[inline(always)]
    pub fn pwidth(&self) -> PWIDTH_R {
        PWIDTH_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Transfer data size of memory"]
    #[inline(always)]
    pub fn mwidth(&self) -> MWIDTH_R {
        MWIDTH_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Priority level"]
    #[inline(always)]
    pub fn prio(&self) -> PRIO_R {
        PRIO_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Memory to Memory Mode"]
    #[inline(always)]
    pub fn m2m(&self) -> M2M_R {
        M2M_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn chen(&mut self) -> CHEN_W<0> {
        CHEN_W::new(self)
    }
    #[doc = "Bit 1 - Enable bit for channel full transfer finish interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ftfie(&mut self) -> FTFIE_W<1> {
        FTFIE_W::new(self)
    }
    #[doc = "Bit 2 - Enable bit for channel half transfer finish interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn htfie(&mut self) -> HTFIE_W<2> {
        HTFIE_W::new(self)
    }
    #[doc = "Bit 3 - Enable bit for channel error interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<3> {
        ERRIE_W::new(self)
    }
    #[doc = "Bit 4 - Transfer direction"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<4> {
        DIR_W::new(self)
    }
    #[doc = "Bit 5 - Circular mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmen(&mut self) -> CMEN_W<5> {
        CMEN_W::new(self)
    }
    #[doc = "Bit 6 - Next address generation algorithm of peripheral"]
    #[inline(always)]
    #[must_use]
    pub fn pnaga(&mut self) -> PNAGA_W<6> {
        PNAGA_W::new(self)
    }
    #[doc = "Bit 7 - Next address generation algorithm of memory"]
    #[inline(always)]
    #[must_use]
    pub fn mnaga(&mut self) -> MNAGA_W<7> {
        MNAGA_W::new(self)
    }
    #[doc = "Bits 8:9 - Transfer data size of peripheral"]
    #[inline(always)]
    #[must_use]
    pub fn pwidth(&mut self) -> PWIDTH_W<8> {
        PWIDTH_W::new(self)
    }
    #[doc = "Bits 10:11 - Transfer data size of memory"]
    #[inline(always)]
    #[must_use]
    pub fn mwidth(&mut self) -> MWIDTH_W<10> {
        MWIDTH_W::new(self)
    }
    #[doc = "Bits 12:13 - Priority level"]
    #[inline(always)]
    #[must_use]
    pub fn prio(&mut self) -> PRIO_W<12> {
        PRIO_W::new(self)
    }
    #[doc = "Bit 14 - Memory to Memory Mode"]
    #[inline(always)]
    #[must_use]
    pub fn m2m(&mut self) -> M2M_W<14> {
        M2M_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel 6 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6ctl](index.html) module"]
pub struct CH6CTL_SPEC;
impl crate::RegisterSpec for CH6CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch6ctl::R](R) reader structure"]
impl crate::Readable for CH6CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch6ctl::W](W) writer structure"]
impl crate::Writable for CH6CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH6CTL to value 0"]
impl crate::Resettable for CH6CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
