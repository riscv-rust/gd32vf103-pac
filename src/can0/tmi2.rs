#[doc = "Register `TMI2` reader"]
pub type R = crate::R<TMI2_SPEC>;
#[doc = "Register `TMI2` writer"]
pub type W = crate::W<TMI2_SPEC>;
#[doc = "Field `TEN` reader - Transmit enable"]
pub type TEN_R = crate::BitReader;
#[doc = "Field `TEN` writer - Transmit enable"]
pub type TEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT` reader - Frame type"]
pub type FT_R = crate::BitReader;
#[doc = "Field `FT` writer - Frame type"]
pub type FT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FF` reader - Frame format"]
pub type FF_R = crate::BitReader;
#[doc = "Field `FF` writer - Frame format"]
pub type FF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFID` reader - The frame identifier"]
pub type EFID_R = crate::FieldReader<u32>;
#[doc = "Field `EFID` writer - The frame identifier"]
pub type EFID_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `SFID_EFID` reader - The frame identifier"]
pub type SFID_EFID_R = crate::FieldReader<u16>;
#[doc = "Field `SFID_EFID` writer - The frame identifier"]
pub type SFID_EFID_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bit 0 - Transmit enable"]
    #[inline(always)]
    pub fn ten(&self) -> TEN_R {
        TEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Frame type"]
    #[inline(always)]
    pub fn ft(&self) -> FT_R {
        FT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Frame format"]
    #[inline(always)]
    pub fn ff(&self) -> FF_R {
        FF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:20 - The frame identifier"]
    #[inline(always)]
    pub fn efid(&self) -> EFID_R {
        EFID_R::new((self.bits >> 3) & 0x0003_ffff)
    }
    #[doc = "Bits 21:31 - The frame identifier"]
    #[inline(always)]
    pub fn sfid_efid(&self) -> SFID_EFID_R {
        SFID_EFID_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit enable"]
    #[inline(always)]
    #[must_use]
    pub fn ten(&mut self) -> TEN_W<TMI2_SPEC> {
        TEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Frame type"]
    #[inline(always)]
    #[must_use]
    pub fn ft(&mut self) -> FT_W<TMI2_SPEC> {
        FT_W::new(self, 1)
    }
    #[doc = "Bit 2 - Frame format"]
    #[inline(always)]
    #[must_use]
    pub fn ff(&mut self) -> FF_W<TMI2_SPEC> {
        FF_W::new(self, 2)
    }
    #[doc = "Bits 3:20 - The frame identifier"]
    #[inline(always)]
    #[must_use]
    pub fn efid(&mut self) -> EFID_W<TMI2_SPEC> {
        EFID_W::new(self, 3)
    }
    #[doc = "Bits 21:31 - The frame identifier"]
    #[inline(always)]
    #[must_use]
    pub fn sfid_efid(&mut self) -> SFID_EFID_W<TMI2_SPEC> {
        SFID_EFID_W::new(self, 21)
    }
}
#[doc = "Transmit mailbox identifier register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`tmi2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmi2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TMI2_SPEC;
impl crate::RegisterSpec for TMI2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmi2::R`](R) reader structure"]
impl crate::Readable for TMI2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tmi2::W`](W) writer structure"]
impl crate::Writable for TMI2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMI2 to value 0"]
impl crate::Resettable for TMI2_SPEC {
    const RESET_VALUE: u32 = 0;
}
