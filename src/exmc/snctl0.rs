#[doc = "Register `SNCTL0` reader"]
pub type R = crate::R<SNCTL0_SPEC>;
#[doc = "Register `SNCTL0` writer"]
pub type W = crate::W<SNCTL0_SPEC>;
#[doc = "Field `NRBKEN` reader - NOR bank enable"]
pub type NRBKEN_R = crate::BitReader;
#[doc = "Field `NRBKEN` writer - NOR bank enable"]
pub type NRBKEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NRMUX` reader - NOR bank memory address/data multiplexing"]
pub type NRMUX_R = crate::BitReader;
#[doc = "Field `NRMUX` writer - NOR bank memory address/data multiplexing"]
pub type NRMUX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NRTP` reader - NOR bank memory type"]
pub type NRTP_R = crate::FieldReader;
#[doc = "Field `NRTP` writer - NOR bank memory type"]
pub type NRTP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `NRW` reader - NOR bank memory data bus width"]
pub type NRW_R = crate::FieldReader;
#[doc = "Field `NRW` writer - NOR bank memory data bus width"]
pub type NRW_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `NREN` reader - NOR Flash access enable"]
pub type NREN_R = crate::BitReader;
#[doc = "Field `NREN` writer - NOR Flash access enable"]
pub type NREN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NRWTPOL` reader - NWAIT signal polarity"]
pub type NRWTPOL_R = crate::BitReader;
#[doc = "Field `NRWTPOL` writer - NWAIT signal polarity"]
pub type NRWTPOL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WREN` reader - Write enable"]
pub type WREN_R = crate::BitReader;
#[doc = "Field `WREN` writer - Write enable"]
pub type WREN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NRWTEN` reader - NWAIT signal enable"]
pub type NRWTEN_R = crate::BitReader;
#[doc = "Field `NRWTEN` writer - NWAIT signal enable"]
pub type NRWTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ASYNCWAIT` reader - Asynchronous wait"]
pub type ASYNCWAIT_R = crate::BitReader;
#[doc = "Field `ASYNCWAIT` writer - Asynchronous wait"]
pub type ASYNCWAIT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - NOR bank enable"]
    #[inline(always)]
    pub fn nrbken(&self) -> NRBKEN_R {
        NRBKEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NOR bank memory address/data multiplexing"]
    #[inline(always)]
    pub fn nrmux(&self) -> NRMUX_R {
        NRMUX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - NOR bank memory type"]
    #[inline(always)]
    pub fn nrtp(&self) -> NRTP_R {
        NRTP_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - NOR bank memory data bus width"]
    #[inline(always)]
    pub fn nrw(&self) -> NRW_R {
        NRW_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - NOR Flash access enable"]
    #[inline(always)]
    pub fn nren(&self) -> NREN_R {
        NREN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - NWAIT signal polarity"]
    #[inline(always)]
    pub fn nrwtpol(&self) -> NRWTPOL_R {
        NRWTPOL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Write enable"]
    #[inline(always)]
    pub fn wren(&self) -> WREN_R {
        WREN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NWAIT signal enable"]
    #[inline(always)]
    pub fn nrwten(&self) -> NRWTEN_R {
        NRWTEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Asynchronous wait"]
    #[inline(always)]
    pub fn asyncwait(&self) -> ASYNCWAIT_R {
        ASYNCWAIT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NOR bank enable"]
    #[inline(always)]
    #[must_use]
    pub fn nrbken(&mut self) -> NRBKEN_W<SNCTL0_SPEC, 0> {
        NRBKEN_W::new(self)
    }
    #[doc = "Bit 1 - NOR bank memory address/data multiplexing"]
    #[inline(always)]
    #[must_use]
    pub fn nrmux(&mut self) -> NRMUX_W<SNCTL0_SPEC, 1> {
        NRMUX_W::new(self)
    }
    #[doc = "Bits 2:3 - NOR bank memory type"]
    #[inline(always)]
    #[must_use]
    pub fn nrtp(&mut self) -> NRTP_W<SNCTL0_SPEC, 2> {
        NRTP_W::new(self)
    }
    #[doc = "Bits 4:5 - NOR bank memory data bus width"]
    #[inline(always)]
    #[must_use]
    pub fn nrw(&mut self) -> NRW_W<SNCTL0_SPEC, 4> {
        NRW_W::new(self)
    }
    #[doc = "Bit 6 - NOR Flash access enable"]
    #[inline(always)]
    #[must_use]
    pub fn nren(&mut self) -> NREN_W<SNCTL0_SPEC, 6> {
        NREN_W::new(self)
    }
    #[doc = "Bit 9 - NWAIT signal polarity"]
    #[inline(always)]
    #[must_use]
    pub fn nrwtpol(&mut self) -> NRWTPOL_W<SNCTL0_SPEC, 9> {
        NRWTPOL_W::new(self)
    }
    #[doc = "Bit 12 - Write enable"]
    #[inline(always)]
    #[must_use]
    pub fn wren(&mut self) -> WREN_W<SNCTL0_SPEC, 12> {
        WREN_W::new(self)
    }
    #[doc = "Bit 13 - NWAIT signal enable"]
    #[inline(always)]
    #[must_use]
    pub fn nrwten(&mut self) -> NRWTEN_W<SNCTL0_SPEC, 13> {
        NRWTEN_W::new(self)
    }
    #[doc = "Bit 15 - Asynchronous wait"]
    #[inline(always)]
    #[must_use]
    pub fn asyncwait(&mut self) -> ASYNCWAIT_W<SNCTL0_SPEC, 15> {
        ASYNCWAIT_W::new(self)
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
#[doc = "SRAM/NOR flash control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`snctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`snctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SNCTL0_SPEC;
impl crate::RegisterSpec for SNCTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`snctl0::R`](R) reader structure"]
impl crate::Readable for SNCTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`snctl0::W`](W) writer structure"]
impl crate::Writable for SNCTL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SNCTL0 to value 0x30da"]
impl crate::Resettable for SNCTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0x30da;
}
