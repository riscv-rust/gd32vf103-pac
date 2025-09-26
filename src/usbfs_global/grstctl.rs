#[doc = "Register `GRSTCTL` reader"]
pub type R = crate::R<GRSTCTL_SPEC>;
#[doc = "Register `GRSTCTL` writer"]
pub type W = crate::W<GRSTCTL_SPEC>;
#[doc = "Field `CSRST` reader - Core soft reset"]
pub type CSRST_R = crate::BitReader;
#[doc = "Field `CSRST` writer - Core soft reset"]
pub type CSRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCSRST` reader - HCLK soft reset"]
pub type HCSRST_R = crate::BitReader;
#[doc = "Field `HCSRST` writer - HCLK soft reset"]
pub type HCSRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFCRST` reader - Host frame counter reset"]
pub type HFCRST_R = crate::BitReader;
#[doc = "Field `HFCRST` writer - Host frame counter reset"]
pub type HFCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFF` reader - RxFIFO flush"]
pub type RXFF_R = crate::BitReader;
#[doc = "Field `RXFF` writer - RxFIFO flush"]
pub type RXFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFF` reader - TxFIFO flush"]
pub type TXFF_R = crate::BitReader;
#[doc = "Field `TXFF` writer - TxFIFO flush"]
pub type TXFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFNUM` reader - TxFIFO number"]
pub type TXFNUM_R = crate::FieldReader;
#[doc = "Field `TXFNUM` writer - TxFIFO number"]
pub type TXFNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - Core soft reset"]
    #[inline(always)]
    pub fn csrst(&self) -> CSRST_R {
        CSRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HCLK soft reset"]
    #[inline(always)]
    pub fn hcsrst(&self) -> HCSRST_R {
        HCSRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Host frame counter reset"]
    #[inline(always)]
    pub fn hfcrst(&self) -> HFCRST_R {
        HFCRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - RxFIFO flush"]
    #[inline(always)]
    pub fn rxff(&self) -> RXFF_R {
        RXFF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TxFIFO flush"]
    #[inline(always)]
    pub fn txff(&self) -> TXFF_R {
        TXFF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:10 - TxFIFO number"]
    #[inline(always)]
    pub fn txfnum(&self) -> TXFNUM_R {
        TXFNUM_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Core soft reset"]
    #[inline(always)]
    #[must_use]
    pub fn csrst(&mut self) -> CSRST_W<GRSTCTL_SPEC> {
        CSRST_W::new(self, 0)
    }
    #[doc = "Bit 1 - HCLK soft reset"]
    #[inline(always)]
    #[must_use]
    pub fn hcsrst(&mut self) -> HCSRST_W<GRSTCTL_SPEC> {
        HCSRST_W::new(self, 1)
    }
    #[doc = "Bit 2 - Host frame counter reset"]
    #[inline(always)]
    #[must_use]
    pub fn hfcrst(&mut self) -> HFCRST_W<GRSTCTL_SPEC> {
        HFCRST_W::new(self, 2)
    }
    #[doc = "Bit 4 - RxFIFO flush"]
    #[inline(always)]
    #[must_use]
    pub fn rxff(&mut self) -> RXFF_W<GRSTCTL_SPEC> {
        RXFF_W::new(self, 4)
    }
    #[doc = "Bit 5 - TxFIFO flush"]
    #[inline(always)]
    #[must_use]
    pub fn txff(&mut self) -> TXFF_W<GRSTCTL_SPEC> {
        TXFF_W::new(self, 5)
    }
    #[doc = "Bits 6:10 - TxFIFO number"]
    #[inline(always)]
    #[must_use]
    pub fn txfnum(&mut self) -> TXFNUM_W<GRSTCTL_SPEC> {
        TXFNUM_W::new(self, 6)
    }
}
#[doc = "Global reset control register (USBFS_GRSTCTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`grstctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grstctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GRSTCTL_SPEC;
impl crate::RegisterSpec for GRSTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grstctl::R`](R) reader structure"]
impl crate::Readable for GRSTCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`grstctl::W`](W) writer structure"]
impl crate::Writable for GRSTCTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRSTCTL to value 0x8000_0000"]
impl crate::Resettable for GRSTCTL_SPEC {
    const RESET_VALUE: u32 = 0x8000_0000;
}
