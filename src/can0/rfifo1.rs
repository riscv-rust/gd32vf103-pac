#[doc = "Register `RFIFO1` reader"]
pub type R = crate::R<RFIFO1_SPEC>;
#[doc = "Register `RFIFO1` writer"]
pub type W = crate::W<RFIFO1_SPEC>;
#[doc = "Field `RFL1` reader - Receive FIFO1 length"]
pub type RFL1_R = crate::FieldReader;
#[doc = "Field `RFF1` reader - Receive FIFO1 full"]
pub type RFF1_R = crate::BitReader;
#[doc = "Field `RFF1` writer - Receive FIFO1 full"]
pub type RFF1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RFO1` reader - Receive FIFO1 overfull"]
pub type RFO1_R = crate::BitReader;
#[doc = "Field `RFO1` writer - Receive FIFO1 overfull"]
pub type RFO1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RFD1` reader - Receive FIFO1 dequeue"]
pub type RFD1_R = crate::BitReader;
#[doc = "Field `RFD1` writer - Receive FIFO1 dequeue"]
pub type RFD1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    pub fn rff1(&mut self) -> RFF1_W<RFIFO1_SPEC, 3> {
        RFF1_W::new(self)
    }
    #[doc = "Bit 4 - Receive FIFO1 overfull"]
    #[inline(always)]
    #[must_use]
    pub fn rfo1(&mut self) -> RFO1_W<RFIFO1_SPEC, 4> {
        RFO1_W::new(self)
    }
    #[doc = "Bit 5 - Receive FIFO1 dequeue"]
    #[inline(always)]
    #[must_use]
    pub fn rfd1(&mut self) -> RFD1_W<RFIFO1_SPEC, 5> {
        RFD1_W::new(self)
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
#[doc = "Receive message FIFO1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfifo1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfifo1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RFIFO1_SPEC;
impl crate::RegisterSpec for RFIFO1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfifo1::R`](R) reader structure"]
impl crate::Readable for RFIFO1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rfifo1::W`](W) writer structure"]
impl crate::Writable for RFIFO1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RFIFO1 to value 0"]
impl crate::Resettable for RFIFO1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
