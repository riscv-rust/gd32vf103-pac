#[doc = "Register `HCH2LEN` reader"]
pub type R = crate::R<HCH2LEN_SPEC>;
#[doc = "Register `HCH2LEN` writer"]
pub type W = crate::W<HCH2LEN_SPEC>;
#[doc = "Field `TLEN` reader - Transfer length"]
pub type TLEN_R = crate::FieldReader<u32>;
#[doc = "Field `TLEN` writer - Transfer length"]
pub type TLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `PCNT` reader - Packet count"]
pub type PCNT_R = crate::FieldReader<u16>;
#[doc = "Field `PCNT` writer - Packet count"]
pub type PCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DPID` reader - Data PID"]
pub type DPID_R = crate::FieldReader;
#[doc = "Field `DPID` writer - Data PID"]
pub type DPID_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
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
    #[doc = "Bits 29:30 - Data PID"]
    #[inline(always)]
    pub fn dpid(&self) -> DPID_R {
        DPID_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:18 - Transfer length"]
    #[inline(always)]
    #[must_use]
    pub fn tlen(&mut self) -> TLEN_W<HCH2LEN_SPEC> {
        TLEN_W::new(self, 0)
    }
    #[doc = "Bits 19:28 - Packet count"]
    #[inline(always)]
    #[must_use]
    pub fn pcnt(&mut self) -> PCNT_W<HCH2LEN_SPEC> {
        PCNT_W::new(self, 19)
    }
    #[doc = "Bits 29:30 - Data PID"]
    #[inline(always)]
    #[must_use]
    pub fn dpid(&mut self) -> DPID_W<HCH2LEN_SPEC> {
        DPID_W::new(self, 29)
    }
}
#[doc = "host channel-2 transfer length register\n\nYou can [`read`](crate::Reg::read) this register and get [`hch2len::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hch2len::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCH2LEN_SPEC;
impl crate::RegisterSpec for HCH2LEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hch2len::R`](R) reader structure"]
impl crate::Readable for HCH2LEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hch2len::W`](W) writer structure"]
impl crate::Writable for HCH2LEN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCH2LEN to value 0"]
impl crate::Resettable for HCH2LEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
