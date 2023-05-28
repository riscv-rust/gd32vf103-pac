#[doc = "Register `RFIFO1` reader"]
pub struct R(crate::R<RFIFO1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFIFO1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFIFO1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFIFO1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RFIFO1` writer"]
pub struct W(crate::W<RFIFO1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFIFO1_SPEC>;
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
impl From<crate::W<RFIFO1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFIFO1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFL1` reader - Receive FIFO1 length"]
pub type RFL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RFF1` reader - Receive FIFO1 full"]
pub type RFF1_R = crate::BitReader<bool>;
#[doc = "Field `RFF1` writer - Receive FIFO1 full"]
pub type RFF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFIFO1_SPEC, bool, O>;
#[doc = "Field `RFO1` reader - Receive FIFO1 overfull"]
pub type RFO1_R = crate::BitReader<bool>;
#[doc = "Field `RFO1` writer - Receive FIFO1 overfull"]
pub type RFO1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFIFO1_SPEC, bool, O>;
#[doc = "Field `RFD1` reader - Receive FIFO1 dequeue"]
pub type RFD1_R = crate::BitReader<bool>;
#[doc = "Field `RFD1` writer - Receive FIFO1 dequeue"]
pub type RFD1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFIFO1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Receive FIFO1 length"]
    #[inline(always)]
    pub fn rfl1(&self) -> RFL1_R {
        RFL1_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Receive FIFO1 full"]
    #[inline(always)]
    pub fn rff1(&self) -> RFF1_R {
        RFF1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO1 overfull"]
    #[inline(always)]
    pub fn rfo1(&self) -> RFO1_R {
        RFO1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive FIFO1 dequeue"]
    #[inline(always)]
    pub fn rfd1(&self) -> RFD1_R {
        RFD1_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Receive FIFO1 full"]
    #[inline(always)]
    #[must_use]
    pub fn rff1(&mut self) -> RFF1_W<3> {
        RFF1_W::new(self)
    }
    #[doc = "Bit 4 - Receive FIFO1 overfull"]
    #[inline(always)]
    #[must_use]
    pub fn rfo1(&mut self) -> RFO1_W<4> {
        RFO1_W::new(self)
    }
    #[doc = "Bit 5 - Receive FIFO1 dequeue"]
    #[inline(always)]
    #[must_use]
    pub fn rfd1(&mut self) -> RFD1_W<5> {
        RFD1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive message FIFO1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfifo1](index.html) module"]
pub struct RFIFO1_SPEC;
impl crate::RegisterSpec for RFIFO1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfifo1::R](R) reader structure"]
impl crate::Readable for RFIFO1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfifo1::W](W) writer structure"]
impl crate::Writable for RFIFO1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RFIFO1 to value 0"]
impl crate::Resettable for RFIFO1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
