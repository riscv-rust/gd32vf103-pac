#[doc = "Register `HCH4INTF` reader"]
pub type R = crate::R<HCH4INTF_SPEC>;
#[doc = "Register `HCH4INTF` writer"]
pub type W = crate::W<HCH4INTF_SPEC>;
#[doc = "Field `TF` reader - Transfer finished"]
pub type TF_R = crate::BitReader;
#[doc = "Field `TF` writer - Transfer finished"]
pub type TF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH` reader - Channel halted"]
pub type CH_R = crate::BitReader;
#[doc = "Field `CH` writer - Channel halted"]
pub type CH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STALL` reader - STALL response received interrupt"]
pub type STALL_R = crate::BitReader;
#[doc = "Field `STALL` writer - STALL response received interrupt"]
pub type STALL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NAK` reader - NAK response received interrupt"]
pub type NAK_R = crate::BitReader;
#[doc = "Field `NAK` writer - NAK response received interrupt"]
pub type NAK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ACK` reader - ACK response received/transmitted interrupt"]
pub type ACK_R = crate::BitReader;
#[doc = "Field `ACK` writer - ACK response received/transmitted interrupt"]
pub type ACK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USBER` reader - USB bus error"]
pub type USBER_R = crate::BitReader;
#[doc = "Field `USBER` writer - USB bus error"]
pub type USBER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BBER` reader - Babble error"]
pub type BBER_R = crate::BitReader;
#[doc = "Field `BBER` writer - Babble error"]
pub type BBER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `REQOVR` reader - Request queue overrun"]
pub type REQOVR_R = crate::BitReader;
#[doc = "Field `REQOVR` writer - Request queue overrun"]
pub type REQOVR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTER` reader - Data toggle error"]
pub type DTER_R = crate::BitReader;
#[doc = "Field `DTER` writer - Data toggle error"]
pub type DTER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Transfer finished"]
    #[inline(always)]
    pub fn tf(&self) -> TF_R {
        TF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel halted"]
    #[inline(always)]
    pub fn ch(&self) -> CH_R {
        CH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - STALL response received interrupt"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAK response received interrupt"]
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ACK response received/transmitted interrupt"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - USB bus error"]
    #[inline(always)]
    pub fn usber(&self) -> USBER_R {
        USBER_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Babble error"]
    #[inline(always)]
    pub fn bber(&self) -> BBER_R {
        BBER_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Request queue overrun"]
    #[inline(always)]
    pub fn reqovr(&self) -> REQOVR_R {
        REQOVR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data toggle error"]
    #[inline(always)]
    pub fn dter(&self) -> DTER_R {
        DTER_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer finished"]
    #[inline(always)]
    #[must_use]
    pub fn tf(&mut self) -> TF_W<HCH4INTF_SPEC, 0> {
        TF_W::new(self)
    }
    #[doc = "Bit 1 - Channel halted"]
    #[inline(always)]
    #[must_use]
    pub fn ch(&mut self) -> CH_W<HCH4INTF_SPEC, 1> {
        CH_W::new(self)
    }
    #[doc = "Bit 3 - STALL response received interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<HCH4INTF_SPEC, 3> {
        STALL_W::new(self)
    }
    #[doc = "Bit 4 - NAK response received interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn nak(&mut self) -> NAK_W<HCH4INTF_SPEC, 4> {
        NAK_W::new(self)
    }
    #[doc = "Bit 5 - ACK response received/transmitted interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> ACK_W<HCH4INTF_SPEC, 5> {
        ACK_W::new(self)
    }
    #[doc = "Bit 7 - USB bus error"]
    #[inline(always)]
    #[must_use]
    pub fn usber(&mut self) -> USBER_W<HCH4INTF_SPEC, 7> {
        USBER_W::new(self)
    }
    #[doc = "Bit 8 - Babble error"]
    #[inline(always)]
    #[must_use]
    pub fn bber(&mut self) -> BBER_W<HCH4INTF_SPEC, 8> {
        BBER_W::new(self)
    }
    #[doc = "Bit 9 - Request queue overrun"]
    #[inline(always)]
    #[must_use]
    pub fn reqovr(&mut self) -> REQOVR_W<HCH4INTF_SPEC, 9> {
        REQOVR_W::new(self)
    }
    #[doc = "Bit 10 - Data toggle error"]
    #[inline(always)]
    #[must_use]
    pub fn dter(&mut self) -> DTER_W<HCH4INTF_SPEC, 10> {
        DTER_W::new(self)
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
#[doc = "host channel-4 interrupt register (HCH4INTF)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch4intf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch4intf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCH4INTF_SPEC;
impl crate::RegisterSpec for HCH4INTF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hch4intf::R`](R) reader structure"]
impl crate::Readable for HCH4INTF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hch4intf::W`](W) writer structure"]
impl crate::Writable for HCH4INTF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCH4INTF to value 0"]
impl crate::Resettable for HCH4INTF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
