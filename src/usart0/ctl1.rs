#[doc = "Register `CTL1` reader"]
pub type R = crate::R<CTL1_SPEC>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<CTL1_SPEC>;
#[doc = "Field `ADDR` reader - Address of the USART"]
pub type ADDR_R = crate::FieldReader;
#[doc = "Field `ADDR` writer - Address of the USART"]
pub type ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LBLEN` reader - LIN break frame length"]
pub type LBLEN_R = crate::BitReader;
#[doc = "Field `LBLEN` writer - LIN break frame length"]
pub type LBLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBDIE` reader - LIN break detection interrupt enable"]
pub type LBDIE_R = crate::BitReader;
#[doc = "Field `LBDIE` writer - LIN break detection interrupt enable"]
pub type LBDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEN` reader - CK Length"]
pub type CLEN_R = crate::BitReader;
#[doc = "Field `CLEN` writer - CK Length"]
pub type CLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPH` reader - Clock phase"]
pub type CPH_R = crate::BitReader;
#[doc = "Field `CPH` writer - Clock phase"]
pub type CPH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPL` reader - Clock polarity"]
pub type CPL_R = crate::BitReader;
#[doc = "Field `CPL` writer - Clock polarity"]
pub type CPL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKEN` reader - CK pin enable"]
pub type CKEN_R = crate::BitReader;
#[doc = "Field `CKEN` writer - CK pin enable"]
pub type CKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STB` reader - STOP bits length"]
pub type STB_R = crate::FieldReader;
#[doc = "Field `STB` writer - STOP bits length"]
pub type STB_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LMEN` reader - LIN mode enable"]
pub type LMEN_R = crate::BitReader;
#[doc = "Field `LMEN` writer - LIN mode enable"]
pub type LMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Address of the USART"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 5 - LIN break frame length"]
    #[inline(always)]
    pub fn lblen(&self) -> LBLEN_R {
        LBLEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable"]
    #[inline(always)]
    pub fn lbdie(&self) -> LBDIE_R {
        LBDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - CK Length"]
    #[inline(always)]
    pub fn clen(&self) -> CLEN_R {
        CLEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock phase"]
    #[inline(always)]
    pub fn cph(&self) -> CPH_R {
        CPH_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock polarity"]
    #[inline(always)]
    pub fn cpl(&self) -> CPL_R {
        CPL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CK pin enable"]
    #[inline(always)]
    pub fn cken(&self) -> CKEN_R {
        CKEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - STOP bits length"]
    #[inline(always)]
    pub fn stb(&self) -> STB_R {
        STB_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline(always)]
    pub fn lmen(&self) -> LMEN_R {
        LMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Address of the USART"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<CTL1_SPEC> {
        ADDR_W::new(self, 0)
    }
    #[doc = "Bit 5 - LIN break frame length"]
    #[inline(always)]
    #[must_use]
    pub fn lblen(&mut self) -> LBLEN_W<CTL1_SPEC> {
        LBLEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lbdie(&mut self) -> LBDIE_W<CTL1_SPEC> {
        LBDIE_W::new(self, 6)
    }
    #[doc = "Bit 8 - CK Length"]
    #[inline(always)]
    #[must_use]
    pub fn clen(&mut self) -> CLEN_W<CTL1_SPEC> {
        CLEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Clock phase"]
    #[inline(always)]
    #[must_use]
    pub fn cph(&mut self) -> CPH_W<CTL1_SPEC> {
        CPH_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cpl(&mut self) -> CPL_W<CTL1_SPEC> {
        CPL_W::new(self, 10)
    }
    #[doc = "Bit 11 - CK pin enable"]
    #[inline(always)]
    #[must_use]
    pub fn cken(&mut self) -> CKEN_W<CTL1_SPEC> {
        CKEN_W::new(self, 11)
    }
    #[doc = "Bits 12:13 - STOP bits length"]
    #[inline(always)]
    #[must_use]
    pub fn stb(&mut self) -> STB_W<CTL1_SPEC> {
        STB_W::new(self, 12)
    }
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn lmen(&mut self) -> LMEN_W<CTL1_SPEC> {
        LMEN_W::new(self, 14)
    }
}
#[doc = "Control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL1_SPEC;
impl crate::RegisterSpec for CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for CTL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for CTL1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for CTL1_SPEC {
    const RESET_VALUE: u32 = 0;
}
