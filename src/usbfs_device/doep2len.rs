#[doc = "Register `DOEP2LEN` reader"]
pub type R = crate::R<DOEP2LEN_SPEC>;
#[doc = "Register `DOEP2LEN` writer"]
pub type W = crate::W<DOEP2LEN_SPEC>;
#[doc = "Field `TLEN` reader - Transfer length"]
pub type TLEN_R = crate::FieldReader<u32>;
#[doc = "Field `TLEN` writer - Transfer length"]
pub type TLEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 19, O, u32>;
#[doc = "Field `PCNT` reader - Packet count"]
pub type PCNT_R = crate::FieldReader<u16>;
#[doc = "Field `PCNT` writer - Packet count"]
pub type PCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
#[doc = "Field `STPCNT_RXDPID` reader - SETUP packet count/Received data PID"]
pub type STPCNT_RXDPID_R = crate::FieldReader;
#[doc = "Field `STPCNT_RXDPID` writer - SETUP packet count/Received data PID"]
pub type STPCNT_RXDPID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:18 - Transfer length"]
    #[inline(always)]
    pub fn tlen(&self) -> TLEN_R {
        TLEN_R::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bits 19:28 - Packet count"]
    #[inline(always)]
    pub fn pcnt(&self) -> PCNT_R {
        PCNT_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:30 - SETUP packet count/Received data PID"]
    #[inline(always)]
    pub fn stpcnt_rxdpid(&self) -> STPCNT_RXDPID_R {
        STPCNT_RXDPID_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:18 - Transfer length"]
    #[inline(always)]
    #[must_use]
    pub fn tlen(&mut self) -> TLEN_W<DOEP2LEN_SPEC, 0> {
        TLEN_W::new(self)
    }
    #[doc = "Bits 19:28 - Packet count"]
    #[inline(always)]
    #[must_use]
    pub fn pcnt(&mut self) -> PCNT_W<DOEP2LEN_SPEC, 19> {
        PCNT_W::new(self)
    }
    #[doc = "Bits 29:30 - SETUP packet count/Received data PID"]
    #[inline(always)]
    #[must_use]
    pub fn stpcnt_rxdpid(&mut self) -> STPCNT_RXDPID_W<DOEP2LEN_SPEC, 29> {
        STPCNT_RXDPID_W::new(self)
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
#[doc = "device OUT endpoint-2 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep2len::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep2len::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEP2LEN_SPEC;
impl crate::RegisterSpec for DOEP2LEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doep2len::R`](R) reader structure"]
impl crate::Readable for DOEP2LEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doep2len::W`](W) writer structure"]
impl crate::Writable for DOEP2LEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEP2LEN to value 0"]
impl crate::Resettable for DOEP2LEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
