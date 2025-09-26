#[doc = "Register `BT` reader"]
pub type R = crate::R<BT_SPEC>;
#[doc = "Register `BT` writer"]
pub type W = crate::W<BT_SPEC>;
#[doc = "Field `BAUDPSC` reader - Baud rate prescaler"]
pub type BAUDPSC_R = crate::FieldReader<u16>;
#[doc = "Field `BAUDPSC` writer - Baud rate prescaler"]
pub type BAUDPSC_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `BS1` reader - Bit segment 1"]
pub type BS1_R = crate::FieldReader;
#[doc = "Field `BS1` writer - Bit segment 1"]
pub type BS1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BS2` reader - Bit segment 2"]
pub type BS2_R = crate::FieldReader;
#[doc = "Field `BS2` writer - Bit segment 2"]
pub type BS2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SJW` reader - Resynchronization jump width"]
pub type SJW_R = crate::FieldReader;
#[doc = "Field `SJW` writer - Resynchronization jump width"]
pub type SJW_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LCMOD` reader - Loopback communication mode"]
pub type LCMOD_R = crate::BitReader;
#[doc = "Field `LCMOD` writer - Loopback communication mode"]
pub type LCMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCMOD` reader - Silent communication mode"]
pub type SCMOD_R = crate::BitReader;
#[doc = "Field `SCMOD` writer - Silent communication mode"]
pub type SCMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - Baud rate prescaler"]
    #[inline(always)]
    pub fn baudpsc(&self) -> BAUDPSC_R {
        BAUDPSC_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:19 - Bit segment 1"]
    #[inline(always)]
    pub fn bs1(&self) -> BS1_R {
        BS1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - Bit segment 2"]
    #[inline(always)]
    pub fn bs2(&self) -> BS2_R {
        BS2_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:25 - Resynchronization jump width"]
    #[inline(always)]
    pub fn sjw(&self) -> SJW_R {
        SJW_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 30 - Loopback communication mode"]
    #[inline(always)]
    pub fn lcmod(&self) -> LCMOD_R {
        LCMOD_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Silent communication mode"]
    #[inline(always)]
    pub fn scmod(&self) -> SCMOD_R {
        SCMOD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Baud rate prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn baudpsc(&mut self) -> BAUDPSC_W<BT_SPEC> {
        BAUDPSC_W::new(self, 0)
    }
    #[doc = "Bits 16:19 - Bit segment 1"]
    #[inline(always)]
    #[must_use]
    pub fn bs1(&mut self) -> BS1_W<BT_SPEC> {
        BS1_W::new(self, 16)
    }
    #[doc = "Bits 20:22 - Bit segment 2"]
    #[inline(always)]
    #[must_use]
    pub fn bs2(&mut self) -> BS2_W<BT_SPEC> {
        BS2_W::new(self, 20)
    }
    #[doc = "Bits 24:25 - Resynchronization jump width"]
    #[inline(always)]
    #[must_use]
    pub fn sjw(&mut self) -> SJW_W<BT_SPEC> {
        SJW_W::new(self, 24)
    }
    #[doc = "Bit 30 - Loopback communication mode"]
    #[inline(always)]
    #[must_use]
    pub fn lcmod(&mut self) -> LCMOD_W<BT_SPEC> {
        LCMOD_W::new(self, 30)
    }
    #[doc = "Bit 31 - Silent communication mode"]
    #[inline(always)]
    #[must_use]
    pub fn scmod(&mut self) -> SCMOD_W<BT_SPEC> {
        SCMOD_W::new(self, 31)
    }
}
#[doc = "Bit timing register\n\nYou can [`read`](crate::Reg::read) this register and get [`bt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BT_SPEC;
impl crate::RegisterSpec for BT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bt::R`](R) reader structure"]
impl crate::Readable for BT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bt::W`](W) writer structure"]
impl crate::Writable for BT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BT to value 0x0123_0000"]
impl crate::Resettable for BT_SPEC {
    const RESET_VALUE: u32 = 0x0123_0000;
}
