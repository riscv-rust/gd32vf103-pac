#[doc = "Register `ERR` reader"]
pub type R = crate::R<ERR_SPEC>;
#[doc = "Register `ERR` writer"]
pub type W = crate::W<ERR_SPEC>;
#[doc = "Field `WERR` reader - Warning error"]
pub type WERR_R = crate::BitReader;
#[doc = "Field `PERR` reader - Passive error"]
pub type PERR_R = crate::BitReader;
#[doc = "Field `BOERR` reader - Bus-off error"]
pub type BOERR_R = crate::BitReader;
#[doc = "Field `ERRN` reader - Error number"]
pub type ERRN_R = crate::FieldReader;
#[doc = "Field `ERRN` writer - Error number"]
pub type ERRN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `TECNT` reader - Transmit Error Count defined by the CAN standard"]
pub type TECNT_R = crate::FieldReader;
#[doc = "Field `RECNT` reader - Receive Error Count defined by the CAN standard"]
pub type RECNT_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Warning error"]
    #[inline(always)]
    pub fn werr(&self) -> WERR_R {
        WERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Passive error"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bus-off error"]
    #[inline(always)]
    pub fn boerr(&self) -> BOERR_R {
        BOERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Error number"]
    #[inline(always)]
    pub fn errn(&self) -> ERRN_R {
        ERRN_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 16:23 - Transmit Error Count defined by the CAN standard"]
    #[inline(always)]
    pub fn tecnt(&self) -> TECNT_R {
        TECNT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Receive Error Count defined by the CAN standard"]
    #[inline(always)]
    pub fn recnt(&self) -> RECNT_R {
        RECNT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - Error number"]
    #[inline(always)]
    #[must_use]
    pub fn errn(&mut self) -> ERRN_W<ERR_SPEC, 4> {
        ERRN_W::new(self)
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
#[doc = "Error register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`err::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ERR_SPEC;
impl crate::RegisterSpec for ERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err::R`](R) reader structure"]
impl crate::Readable for ERR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`err::W`](W) writer structure"]
impl crate::Writable for ERR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ERR to value 0"]
impl crate::Resettable for ERR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
