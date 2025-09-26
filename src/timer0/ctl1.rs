#[doc = "Register `CTL1` reader"]
pub type R = crate::R<CTL1_SPEC>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<CTL1_SPEC>;
#[doc = "Field `CCSE` reader - Commutation control shadow enable"]
pub type CCSE_R = crate::BitReader;
#[doc = "Field `CCSE` writer - Commutation control shadow enable"]
pub type CCSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CCUC` reader - Commutation control shadow register update control"]
pub type CCUC_R = crate::BitReader;
#[doc = "Field `CCUC` writer - Commutation control shadow register update control"]
pub type CCUC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMAS` reader - DMA request source selection"]
pub type DMAS_R = crate::BitReader;
#[doc = "Field `DMAS` writer - DMA request source selection"]
pub type DMAS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MMC` reader - Master mode control"]
pub type MMC_R = crate::FieldReader;
#[doc = "Field `MMC` writer - Master mode control"]
pub type MMC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `TI0S` reader - Channel 0 trigger input selection"]
pub type TI0S_R = crate::BitReader;
#[doc = "Field `TI0S` writer - Channel 0 trigger input selection"]
pub type TI0S_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ISO0` reader - Idle state of channel 0 output"]
pub type ISO0_R = crate::BitReader;
#[doc = "Field `ISO0` writer - Idle state of channel 0 output"]
pub type ISO0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ISO0N` reader - Idle state of channel 0 complementary output"]
pub type ISO0N_R = crate::BitReader;
#[doc = "Field `ISO0N` writer - Idle state of channel 0 complementary output"]
pub type ISO0N_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ISO1` reader - Idle state of channel 1 output"]
pub type ISO1_R = crate::BitReader;
#[doc = "Field `ISO1` writer - Idle state of channel 1 output"]
pub type ISO1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ISO1N` reader - Idle state of channel 1 complementary output"]
pub type ISO1N_R = crate::BitReader;
#[doc = "Field `ISO1N` writer - Idle state of channel 1 complementary output"]
pub type ISO1N_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ISO2` reader - Idle state of channel 2 output"]
pub type ISO2_R = crate::BitReader;
#[doc = "Field `ISO2` writer - Idle state of channel 2 output"]
pub type ISO2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ISO2N` reader - Idle state of channel 2 complementary output"]
pub type ISO2N_R = crate::BitReader;
#[doc = "Field `ISO2N` writer - Idle state of channel 2 complementary output"]
pub type ISO2N_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ISO3` reader - Idle state of channel 3 output"]
pub type ISO3_R = crate::BitReader;
#[doc = "Field `ISO3` writer - Idle state of channel 3 output"]
pub type ISO3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Commutation control shadow enable"]
    #[inline(always)]
    pub fn ccse(&self) -> CCSE_R {
        CCSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Commutation control shadow register update control"]
    #[inline(always)]
    pub fn ccuc(&self) -> CCUC_R {
        CCUC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA request source selection"]
    #[inline(always)]
    pub fn dmas(&self) -> DMAS_R {
        DMAS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Master mode control"]
    #[inline(always)]
    pub fn mmc(&self) -> MMC_R {
        MMC_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Channel 0 trigger input selection"]
    #[inline(always)]
    pub fn ti0s(&self) -> TI0S_R {
        TI0S_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Idle state of channel 0 output"]
    #[inline(always)]
    pub fn iso0(&self) -> ISO0_R {
        ISO0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Idle state of channel 0 complementary output"]
    #[inline(always)]
    pub fn iso0n(&self) -> ISO0N_R {
        ISO0N_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Idle state of channel 1 output"]
    #[inline(always)]
    pub fn iso1(&self) -> ISO1_R {
        ISO1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Idle state of channel 1 complementary output"]
    #[inline(always)]
    pub fn iso1n(&self) -> ISO1N_R {
        ISO1N_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Idle state of channel 2 output"]
    #[inline(always)]
    pub fn iso2(&self) -> ISO2_R {
        ISO2_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Idle state of channel 2 complementary output"]
    #[inline(always)]
    pub fn iso2n(&self) -> ISO2N_R {
        ISO2N_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Idle state of channel 3 output"]
    #[inline(always)]
    pub fn iso3(&self) -> ISO3_R {
        ISO3_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Commutation control shadow enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccse(&mut self) -> CCSE_W<CTL1_SPEC, 0> {
        CCSE_W::new(self)
    }
    #[doc = "Bit 2 - Commutation control shadow register update control"]
    #[inline(always)]
    #[must_use]
    pub fn ccuc(&mut self) -> CCUC_W<CTL1_SPEC, 2> {
        CCUC_W::new(self)
    }
    #[doc = "Bit 3 - DMA request source selection"]
    #[inline(always)]
    #[must_use]
    pub fn dmas(&mut self) -> DMAS_W<CTL1_SPEC, 3> {
        DMAS_W::new(self)
    }
    #[doc = "Bits 4:6 - Master mode control"]
    #[inline(always)]
    #[must_use]
    pub fn mmc(&mut self) -> MMC_W<CTL1_SPEC, 4> {
        MMC_W::new(self)
    }
    #[doc = "Bit 7 - Channel 0 trigger input selection"]
    #[inline(always)]
    #[must_use]
    pub fn ti0s(&mut self) -> TI0S_W<CTL1_SPEC, 7> {
        TI0S_W::new(self)
    }
    #[doc = "Bit 8 - Idle state of channel 0 output"]
    #[inline(always)]
    #[must_use]
    pub fn iso0(&mut self) -> ISO0_W<CTL1_SPEC, 8> {
        ISO0_W::new(self)
    }
    #[doc = "Bit 9 - Idle state of channel 0 complementary output"]
    #[inline(always)]
    #[must_use]
    pub fn iso0n(&mut self) -> ISO0N_W<CTL1_SPEC, 9> {
        ISO0N_W::new(self)
    }
    #[doc = "Bit 10 - Idle state of channel 1 output"]
    #[inline(always)]
    #[must_use]
    pub fn iso1(&mut self) -> ISO1_W<CTL1_SPEC, 10> {
        ISO1_W::new(self)
    }
    #[doc = "Bit 11 - Idle state of channel 1 complementary output"]
    #[inline(always)]
    #[must_use]
    pub fn iso1n(&mut self) -> ISO1N_W<CTL1_SPEC, 11> {
        ISO1N_W::new(self)
    }
    #[doc = "Bit 12 - Idle state of channel 2 output"]
    #[inline(always)]
    #[must_use]
    pub fn iso2(&mut self) -> ISO2_W<CTL1_SPEC, 12> {
        ISO2_W::new(self)
    }
    #[doc = "Bit 13 - Idle state of channel 2 complementary output"]
    #[inline(always)]
    #[must_use]
    pub fn iso2n(&mut self) -> ISO2N_W<CTL1_SPEC, 13> {
        ISO2N_W::new(self)
    }
    #[doc = "Bit 14 - Idle state of channel 3 output"]
    #[inline(always)]
    #[must_use]
    pub fn iso3(&mut self) -> ISO3_W<CTL1_SPEC, 14> {
        ISO3_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL1_SPEC;
impl crate::RegisterSpec for CTL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for CTL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for CTL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for CTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
