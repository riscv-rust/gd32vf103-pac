#[doc = "Register `INTF` reader"]
pub type R = crate::R<INTF_SPEC>;
#[doc = "Register `INTF` writer"]
pub type W = crate::W<INTF_SPEC>;
#[doc = "Field `UPIF` reader - Update interrupt flag"]
pub type UPIF_R = crate::BitReader;
#[doc = "Field `UPIF` writer - Update interrupt flag"]
pub type UPIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0IF` reader - Channel 0 capture/compare interrupt flag"]
pub type CH0IF_R = crate::BitReader;
#[doc = "Field `CH0IF` writer - Channel 0 capture/compare interrupt flag"]
pub type CH0IF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1IF` reader - Channel 1 capture/compare interrupt flag"]
pub type CH1IF_R = crate::BitReader;
#[doc = "Field `CH1IF` writer - Channel 1 capture/compare interrupt flag"]
pub type CH1IF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2IF` reader - Channel 2 capture/compare interrupt enable"]
pub type CH2IF_R = crate::BitReader;
#[doc = "Field `CH2IF` writer - Channel 2 capture/compare interrupt enable"]
pub type CH2IF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3IF` reader - Channel 3 capture/compare interrupt enable"]
pub type CH3IF_R = crate::BitReader;
#[doc = "Field `CH3IF` writer - Channel 3 capture/compare interrupt enable"]
pub type CH3IF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGIF` reader - Trigger interrupt flag"]
pub type TRGIF_R = crate::BitReader;
#[doc = "Field `TRGIF` writer - Trigger interrupt flag"]
pub type TRGIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0OF` reader - Channel 0 over capture flag"]
pub type CH0OF_R = crate::BitReader;
#[doc = "Field `CH0OF` writer - Channel 0 over capture flag"]
pub type CH0OF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1OF` reader - Channel 1 over capture flag"]
pub type CH1OF_R = crate::BitReader;
#[doc = "Field `CH1OF` writer - Channel 1 over capture flag"]
pub type CH1OF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2OF` reader - Channel 2 over capture flag"]
pub type CH2OF_R = crate::BitReader;
#[doc = "Field `CH2OF` writer - Channel 2 over capture flag"]
pub type CH2OF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3OF` reader - Channel 3 over capture flag"]
pub type CH3OF_R = crate::BitReader;
#[doc = "Field `CH3OF` writer - Channel 3 over capture flag"]
pub type CH3OF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline(always)]
    pub fn upif(&self) -> UPIF_R {
        UPIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 0 capture/compare interrupt flag"]
    #[inline(always)]
    pub fn ch0if(&self) -> CH0IF_R {
        CH0IF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 1 capture/compare interrupt flag"]
    #[inline(always)]
    pub fn ch1if(&self) -> CH1IF_R {
        CH1IF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 2 capture/compare interrupt enable"]
    #[inline(always)]
    pub fn ch2if(&self) -> CH2IF_R {
        CH2IF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 3 capture/compare interrupt enable"]
    #[inline(always)]
    pub fn ch3if(&self) -> CH3IF_R {
        CH3IF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger interrupt flag"]
    #[inline(always)]
    pub fn trgif(&self) -> TRGIF_R {
        TRGIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 0 over capture flag"]
    #[inline(always)]
    pub fn ch0of(&self) -> CH0OF_R {
        CH0OF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 1 over capture flag"]
    #[inline(always)]
    pub fn ch1of(&self) -> CH1OF_R {
        CH1OF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 2 over capture flag"]
    #[inline(always)]
    pub fn ch2of(&self) -> CH2OF_R {
        CH2OF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 3 over capture flag"]
    #[inline(always)]
    pub fn ch3of(&self) -> CH3OF_R {
        CH3OF_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn upif(&mut self) -> UPIF_W<INTF_SPEC> {
        UPIF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 0 capture/compare interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch0if(&mut self) -> CH0IF_W<INTF_SPEC> {
        CH0IF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 1 capture/compare interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch1if(&mut self) -> CH1IF_W<INTF_SPEC> {
        CH1IF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 2 capture/compare interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2if(&mut self) -> CH2IF_W<INTF_SPEC> {
        CH2IF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 3 capture/compare interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch3if(&mut self) -> CH3IF_W<INTF_SPEC> {
        CH3IF_W::new(self, 4)
    }
    #[doc = "Bit 6 - Trigger interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn trgif(&mut self) -> TRGIF_W<INTF_SPEC> {
        TRGIF_W::new(self, 6)
    }
    #[doc = "Bit 9 - Channel 0 over capture flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch0of(&mut self) -> CH0OF_W<INTF_SPEC> {
        CH0OF_W::new(self, 9)
    }
    #[doc = "Bit 10 - Channel 1 over capture flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch1of(&mut self) -> CH1OF_W<INTF_SPEC> {
        CH1OF_W::new(self, 10)
    }
    #[doc = "Bit 11 - Channel 2 over capture flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch2of(&mut self) -> CH2OF_W<INTF_SPEC> {
        CH2OF_W::new(self, 11)
    }
    #[doc = "Bit 12 - Channel 3 over capture flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch3of(&mut self) -> CH3OF_W<INTF_SPEC> {
        CH3OF_W::new(self, 12)
    }
}
#[doc = "interrupt flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`intf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTF_SPEC;
impl crate::RegisterSpec for INTF_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`intf::R`](R) reader structure"]
impl crate::Readable for INTF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intf::W`](W) writer structure"]
impl crate::Writable for INTF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets INTF to value 0"]
impl crate::Resettable for INTF_SPEC {
    const RESET_VALUE: u16 = 0;
}
