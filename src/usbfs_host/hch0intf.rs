#[doc = "Register `HCH0INTF` reader"]
pub type R = crate::R<HCH0INTF_SPEC>;
#[doc = "Register `HCH0INTF` writer"]
pub type W = crate::W<HCH0INTF_SPEC>;
#[doc = "Field `TF` reader - Transfer finished"]
pub type TF_R = crate::BitReader;
#[doc = "Field `TF` writer - Transfer finished"]
pub type TF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH` reader - Channel halted"]
pub type CH_R = crate::BitReader;
#[doc = "Field `CH` writer - Channel halted"]
pub type CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL` reader - STALL response received interrupt"]
pub type STALL_R = crate::BitReader;
#[doc = "Field `STALL` writer - STALL response received interrupt"]
pub type STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAK` reader - NAK response received interrupt"]
pub type NAK_R = crate::BitReader;
#[doc = "Field `NAK` writer - NAK response received interrupt"]
pub type NAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK` reader - ACK response received/transmitted interrupt"]
pub type ACK_R = crate::BitReader;
#[doc = "Field `ACK` writer - ACK response received/transmitted interrupt"]
pub type ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBER` reader - USB bus error"]
pub type USBER_R = crate::BitReader;
#[doc = "Field `USBER` writer - USB bus error"]
pub type USBER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBER` reader - Babble error"]
pub type BBER_R = crate::BitReader;
#[doc = "Field `BBER` writer - Babble error"]
pub type BBER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REQOVR` reader - Request queue overrun"]
pub type REQOVR_R = crate::BitReader;
#[doc = "Field `REQOVR` writer - Request queue overrun"]
pub type REQOVR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTER` reader - Data toggle error"]
pub type DTER_R = crate::BitReader;
#[doc = "Field `DTER` writer - Data toggle error"]
pub type DTER_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn tf(&mut self) -> TF_W<HCH0INTF_SPEC> {
        TF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel halted"]
    #[inline(always)]
    #[must_use]
    pub fn ch(&mut self) -> CH_W<HCH0INTF_SPEC> {
        CH_W::new(self, 1)
    }
    #[doc = "Bit 3 - STALL response received interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<HCH0INTF_SPEC> {
        STALL_W::new(self, 3)
    }
    #[doc = "Bit 4 - NAK response received interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn nak(&mut self) -> NAK_W<HCH0INTF_SPEC> {
        NAK_W::new(self, 4)
    }
    #[doc = "Bit 5 - ACK response received/transmitted interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> ACK_W<HCH0INTF_SPEC> {
        ACK_W::new(self, 5)
    }
    #[doc = "Bit 7 - USB bus error"]
    #[inline(always)]
    #[must_use]
    pub fn usber(&mut self) -> USBER_W<HCH0INTF_SPEC> {
        USBER_W::new(self, 7)
    }
    #[doc = "Bit 8 - Babble error"]
    #[inline(always)]
    #[must_use]
    pub fn bber(&mut self) -> BBER_W<HCH0INTF_SPEC> {
        BBER_W::new(self, 8)
    }
    #[doc = "Bit 9 - Request queue overrun"]
    #[inline(always)]
    #[must_use]
    pub fn reqovr(&mut self) -> REQOVR_W<HCH0INTF_SPEC> {
        REQOVR_W::new(self, 9)
    }
    #[doc = "Bit 10 - Data toggle error"]
    #[inline(always)]
    #[must_use]
    pub fn dter(&mut self) -> DTER_W<HCH0INTF_SPEC> {
        DTER_W::new(self, 10)
    }
}
#[doc = "host channel-0 interrupt register (USBFS_HCHxINTF)\n\nYou can [`read`](crate::Reg::read) this register and get [`hch0intf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hch0intf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCH0INTF_SPEC;
impl crate::RegisterSpec for HCH0INTF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hch0intf::R`](R) reader structure"]
impl crate::Readable for HCH0INTF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hch0intf::W`](W) writer structure"]
impl crate::Writable for HCH0INTF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCH0INTF to value 0"]
impl crate::Resettable for HCH0INTF_SPEC {
    const RESET_VALUE: u32 = 0;
}
