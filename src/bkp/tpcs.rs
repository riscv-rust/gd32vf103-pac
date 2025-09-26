#[doc = "Register `TPCS` reader"]
pub type R = crate::R<TPCS_SPEC>;
#[doc = "Register `TPCS` writer"]
pub type W = crate::W<TPCS_SPEC>;
#[doc = "Field `TER` reader - Tamper event reset"]
pub type TER_R = crate::BitReader;
#[doc = "Field `TER` writer - Tamper event reset"]
pub type TER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIR` reader - Tamper interrupt reset"]
pub type TIR_R = crate::BitReader;
#[doc = "Field `TIR` writer - Tamper interrupt reset"]
pub type TIR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TPIE` reader - Tamper interrupt enable"]
pub type TPIE_R = crate::BitReader;
#[doc = "Field `TPIE` writer - Tamper interrupt enable"]
pub type TPIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TEF` reader - Tamper event flag"]
pub type TEF_R = crate::BitReader;
#[doc = "Field `TEF` writer - Tamper event flag"]
pub type TEF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIF` reader - Tamper interrupt flag"]
pub type TIF_R = crate::BitReader;
#[doc = "Field `TIF` writer - Tamper interrupt flag"]
pub type TIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Tamper event reset"]
    #[inline(always)]
    pub fn ter(&self) -> TER_R {
        TER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tamper interrupt reset"]
    #[inline(always)]
    pub fn tir(&self) -> TIR_R {
        TIR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Tamper interrupt enable"]
    #[inline(always)]
    pub fn tpie(&self) -> TPIE_R {
        TPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Tamper event flag"]
    #[inline(always)]
    pub fn tef(&self) -> TEF_R {
        TEF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Tamper interrupt flag"]
    #[inline(always)]
    pub fn tif(&self) -> TIF_R {
        TIF_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper event reset"]
    #[inline(always)]
    #[must_use]
    pub fn ter(&mut self) -> TER_W<TPCS_SPEC, 0> {
        TER_W::new(self)
    }
    #[doc = "Bit 1 - Tamper interrupt reset"]
    #[inline(always)]
    #[must_use]
    pub fn tir(&mut self) -> TIR_W<TPCS_SPEC, 1> {
        TIR_W::new(self)
    }
    #[doc = "Bit 2 - Tamper interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tpie(&mut self) -> TPIE_W<TPCS_SPEC, 2> {
        TPIE_W::new(self)
    }
    #[doc = "Bit 8 - Tamper event flag"]
    #[inline(always)]
    #[must_use]
    pub fn tef(&mut self) -> TEF_W<TPCS_SPEC, 8> {
        TEF_W::new(self)
    }
    #[doc = "Bit 9 - Tamper interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn tif(&mut self) -> TIF_W<TPCS_SPEC, 9> {
        TIF_W::new(self)
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
#[doc = "Tamper control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tpcs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tpcs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TPCS_SPEC;
impl crate::RegisterSpec for TPCS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tpcs::R`](R) reader structure"]
impl crate::Readable for TPCS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tpcs::W`](W) writer structure"]
impl crate::Writable for TPCS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TPCS to value 0"]
impl crate::Resettable for TPCS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
