#[doc = "Register `INTF` reader"]
pub type R = crate::R<INTF_SPEC>;
#[doc = "Field `GIF0` reader - Global interrupt flag of channel 0"]
pub type GIF0_R = crate::BitReader;
#[doc = "Field `FTFIF0` reader - Full Transfer finish flag of channe 0"]
pub type FTFIF0_R = crate::BitReader;
#[doc = "Field `HTFIF0` reader - Half transfer finish flag of channel 0"]
pub type HTFIF0_R = crate::BitReader;
#[doc = "Field `ERRIF0` reader - Error flag of channel 0"]
pub type ERRIF0_R = crate::BitReader;
#[doc = "Field `GIF1` reader - Global interrupt flag of channel 1"]
pub type GIF1_R = crate::BitReader;
#[doc = "Field `FTFIF1` reader - Full Transfer finish flag of channe 1"]
pub type FTFIF1_R = crate::BitReader;
#[doc = "Field `HTFIF1` reader - Half transfer finish flag of channel 1"]
pub type HTFIF1_R = crate::BitReader;
#[doc = "Field `ERRIF1` reader - Error flag of channel 1"]
pub type ERRIF1_R = crate::BitReader;
#[doc = "Field `GIF2` reader - Global interrupt flag of channel 2"]
pub type GIF2_R = crate::BitReader;
#[doc = "Field `FTFIF2` reader - Full Transfer finish flag of channe 2"]
pub type FTFIF2_R = crate::BitReader;
#[doc = "Field `HTFIF2` reader - Half transfer finish flag of channel 2"]
pub type HTFIF2_R = crate::BitReader;
#[doc = "Field `ERRIF2` reader - Error flag of channel 2"]
pub type ERRIF2_R = crate::BitReader;
#[doc = "Field `GIF3` reader - Global interrupt flag of channel 3"]
pub type GIF3_R = crate::BitReader;
#[doc = "Field `FTFIF3` reader - Full Transfer finish flag of channe 3"]
pub type FTFIF3_R = crate::BitReader;
#[doc = "Field `HTFIF3` reader - Half transfer finish flag of channel 3"]
pub type HTFIF3_R = crate::BitReader;
#[doc = "Field `ERRIF3` reader - Error flag of channel 3"]
pub type ERRIF3_R = crate::BitReader;
#[doc = "Field `GIF4` reader - Global interrupt flag of channel 4"]
pub type GIF4_R = crate::BitReader;
#[doc = "Field `FTFIF4` reader - Full Transfer finish flag of channe 4"]
pub type FTFIF4_R = crate::BitReader;
#[doc = "Field `HTFIF4` reader - Half transfer finish flag of channel 4"]
pub type HTFIF4_R = crate::BitReader;
#[doc = "Field `ERRIF4` reader - Error flag of channel 4"]
pub type ERRIF4_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Global interrupt flag of channel 0"]
    #[inline(always)]
    pub fn gif0(&self) -> GIF0_R {
        GIF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Full Transfer finish flag of channe 0"]
    #[inline(always)]
    pub fn ftfif0(&self) -> FTFIF0_R {
        FTFIF0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Half transfer finish flag of channel 0"]
    #[inline(always)]
    pub fn htfif0(&self) -> HTFIF0_R {
        HTFIF0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Error flag of channel 0"]
    #[inline(always)]
    pub fn errif0(&self) -> ERRIF0_R {
        ERRIF0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Global interrupt flag of channel 1"]
    #[inline(always)]
    pub fn gif1(&self) -> GIF1_R {
        GIF1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Full Transfer finish flag of channe 1"]
    #[inline(always)]
    pub fn ftfif1(&self) -> FTFIF1_R {
        FTFIF1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Half transfer finish flag of channel 1"]
    #[inline(always)]
    pub fn htfif1(&self) -> HTFIF1_R {
        HTFIF1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Error flag of channel 1"]
    #[inline(always)]
    pub fn errif1(&self) -> ERRIF1_R {
        ERRIF1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Global interrupt flag of channel 2"]
    #[inline(always)]
    pub fn gif2(&self) -> GIF2_R {
        GIF2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Full Transfer finish flag of channe 2"]
    #[inline(always)]
    pub fn ftfif2(&self) -> FTFIF2_R {
        FTFIF2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Half transfer finish flag of channel 2"]
    #[inline(always)]
    pub fn htfif2(&self) -> HTFIF2_R {
        HTFIF2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Error flag of channel 2"]
    #[inline(always)]
    pub fn errif2(&self) -> ERRIF2_R {
        ERRIF2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Global interrupt flag of channel 3"]
    #[inline(always)]
    pub fn gif3(&self) -> GIF3_R {
        GIF3_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Full Transfer finish flag of channe 3"]
    #[inline(always)]
    pub fn ftfif3(&self) -> FTFIF3_R {
        FTFIF3_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Half transfer finish flag of channel 3"]
    #[inline(always)]
    pub fn htfif3(&self) -> HTFIF3_R {
        HTFIF3_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Error flag of channel 3"]
    #[inline(always)]
    pub fn errif3(&self) -> ERRIF3_R {
        ERRIF3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Global interrupt flag of channel 4"]
    #[inline(always)]
    pub fn gif4(&self) -> GIF4_R {
        GIF4_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Full Transfer finish flag of channe 4"]
    #[inline(always)]
    pub fn ftfif4(&self) -> FTFIF4_R {
        FTFIF4_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Half transfer finish flag of channel 4"]
    #[inline(always)]
    pub fn htfif4(&self) -> HTFIF4_R {
        HTFIF4_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Error flag of channel 4"]
    #[inline(always)]
    pub fn errif4(&self) -> ERRIF4_R {
        ERRIF4_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[doc = "Interrupt flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`intf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTF_SPEC;
impl crate::RegisterSpec for INTF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf::R`](R) reader structure"]
impl crate::Readable for INTF_SPEC {}
#[doc = "`reset()` method sets INTF to value 0"]
impl crate::Resettable for INTF_SPEC {
    const RESET_VALUE: u32 = 0;
}
