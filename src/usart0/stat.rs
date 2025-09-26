#[doc = "Register `STAT` reader"]
pub type R = crate::R<STAT_SPEC>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<STAT_SPEC>;
#[doc = "Field `PERR` reader - Parity error flag"]
pub type PERR_R = crate::BitReader;
#[doc = "Field `FERR` reader - Frame error flag"]
pub type FERR_R = crate::BitReader;
#[doc = "Field `NERR` reader - Noise error flag"]
pub type NERR_R = crate::BitReader;
#[doc = "Field `ORERR` reader - Overrun error"]
pub type ORERR_R = crate::BitReader;
#[doc = "Field `IDLEF` reader - IDLE frame detected flag"]
pub type IDLEF_R = crate::BitReader;
#[doc = "Field `RBNE` reader - Read data buffer not empty"]
pub type RBNE_R = crate::BitReader;
#[doc = "Field `RBNE` writer - Read data buffer not empty"]
pub type RBNE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC` reader - Transmission complete"]
pub type TC_R = crate::BitReader;
#[doc = "Field `TC` writer - Transmission complete"]
pub type TC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBE` reader - Transmit data buffer empty"]
pub type TBE_R = crate::BitReader;
#[doc = "Field `LBDF` reader - LIN break detection flag"]
pub type LBDF_R = crate::BitReader;
#[doc = "Field `LBDF` writer - LIN break detection flag"]
pub type LBDF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSF` reader - CTS change flag"]
pub type CTSF_R = crate::BitReader;
#[doc = "Field `CTSF` writer - CTS change flag"]
pub type CTSF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Parity error flag"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Frame error flag"]
    #[inline(always)]
    pub fn ferr(&self) -> FERR_R {
        FERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Noise error flag"]
    #[inline(always)]
    pub fn nerr(&self) -> NERR_R {
        NERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun error"]
    #[inline(always)]
    pub fn orerr(&self) -> ORERR_R {
        ORERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IDLE frame detected flag"]
    #[inline(always)]
    pub fn idlef(&self) -> IDLEF_R {
        IDLEF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Read data buffer not empty"]
    #[inline(always)]
    pub fn rbne(&self) -> RBNE_R {
        RBNE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission complete"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit data buffer empty"]
    #[inline(always)]
    pub fn tbe(&self) -> TBE_R {
        TBE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LIN break detection flag"]
    #[inline(always)]
    pub fn lbdf(&self) -> LBDF_R {
        LBDF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTS change flag"]
    #[inline(always)]
    pub fn ctsf(&self) -> CTSF_R {
        CTSF_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Read data buffer not empty"]
    #[inline(always)]
    #[must_use]
    pub fn rbne(&mut self) -> RBNE_W<STAT_SPEC> {
        RBNE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Transmission complete"]
    #[inline(always)]
    #[must_use]
    pub fn tc(&mut self) -> TC_W<STAT_SPEC> {
        TC_W::new(self, 6)
    }
    #[doc = "Bit 8 - LIN break detection flag"]
    #[inline(always)]
    #[must_use]
    pub fn lbdf(&mut self) -> LBDF_W<STAT_SPEC> {
        LBDF_W::new(self, 8)
    }
    #[doc = "Bit 9 - CTS change flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctsf(&mut self) -> CTSF_W<STAT_SPEC> {
        CTSF_W::new(self, 9)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for STAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STAT to value 0xc0"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: u32 = 0xc0;
}
