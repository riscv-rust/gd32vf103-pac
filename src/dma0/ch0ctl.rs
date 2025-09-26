#[doc = "Register `CH0CTL` reader"]
pub type R = crate::R<CH0CTL_SPEC>;
#[doc = "Register `CH0CTL` writer"]
pub type W = crate::W<CH0CTL_SPEC>;
#[doc = "Field `CHEN` reader - Channel enable"]
pub type CHEN_R = crate::BitReader;
#[doc = "Field `CHEN` writer - Channel enable"]
pub type CHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTFIE` reader - Enable bit for channel full transfer finish interrupt"]
pub type FTFIE_R = crate::BitReader;
#[doc = "Field `FTFIE` writer - Enable bit for channel full transfer finish interrupt"]
pub type FTFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTFIE` reader - Enable bit for channel half transfer finish interrupt"]
pub type HTFIE_R = crate::BitReader;
#[doc = "Field `HTFIE` writer - Enable bit for channel half transfer finish interrupt"]
pub type HTFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIE` reader - Enable bit for channel error interrupt"]
pub type ERRIE_R = crate::BitReader;
#[doc = "Field `ERRIE` writer - Enable bit for channel error interrupt"]
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIR` reader - Transfer direction"]
pub type DIR_R = crate::BitReader;
#[doc = "Field `DIR` writer - Transfer direction"]
pub type DIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMEN` reader - Circular mode enable"]
pub type CMEN_R = crate::BitReader;
#[doc = "Field `CMEN` writer - Circular mode enable"]
pub type CMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PNAGA` reader - Next address generation algorithm of peripheral"]
pub type PNAGA_R = crate::BitReader;
#[doc = "Field `PNAGA` writer - Next address generation algorithm of peripheral"]
pub type PNAGA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MNAGA` reader - Next address generation algorithm of memory"]
pub type MNAGA_R = crate::BitReader;
#[doc = "Field `MNAGA` writer - Next address generation algorithm of memory"]
pub type MNAGA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWIDTH` reader - Transfer data size of peripheral"]
pub type PWIDTH_R = crate::FieldReader;
#[doc = "Field `PWIDTH` writer - Transfer data size of peripheral"]
pub type PWIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MWIDTH` reader - Transfer data size of memory"]
pub type MWIDTH_R = crate::FieldReader;
#[doc = "Field `MWIDTH` writer - Transfer data size of memory"]
pub type MWIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRIO` reader - Priority level"]
pub type PRIO_R = crate::FieldReader;
#[doc = "Field `PRIO` writer - Priority level"]
pub type PRIO_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `M2M` reader - Memory to Memory Mode"]
pub type M2M_R = crate::BitReader;
#[doc = "Field `M2M` writer - Memory to Memory Mode"]
pub type M2M_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel enable"]
    #[inline(always)]
    pub fn chen(&self) -> CHEN_R {
        CHEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable bit for channel full transfer finish interrupt"]
    #[inline(always)]
    pub fn ftfie(&self) -> FTFIE_R {
        FTFIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable bit for channel half transfer finish interrupt"]
    #[inline(always)]
    pub fn htfie(&self) -> HTFIE_R {
        HTFIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable bit for channel error interrupt"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transfer direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Circular mode enable"]
    #[inline(always)]
    pub fn cmen(&self) -> CMEN_R {
        CMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Next address generation algorithm of peripheral"]
    #[inline(always)]
    pub fn pnaga(&self) -> PNAGA_R {
        PNAGA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Next address generation algorithm of memory"]
    #[inline(always)]
    pub fn mnaga(&self) -> MNAGA_R {
        MNAGA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Transfer data size of peripheral"]
    #[inline(always)]
    pub fn pwidth(&self) -> PWIDTH_R {
        PWIDTH_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Transfer data size of memory"]
    #[inline(always)]
    pub fn mwidth(&self) -> MWIDTH_R {
        MWIDTH_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Priority level"]
    #[inline(always)]
    pub fn prio(&self) -> PRIO_R {
        PRIO_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Memory to Memory Mode"]
    #[inline(always)]
    pub fn m2m(&self) -> M2M_R {
        M2M_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn chen(&mut self) -> CHEN_W<CH0CTL_SPEC> {
        CHEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable bit for channel full transfer finish interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ftfie(&mut self) -> FTFIE_W<CH0CTL_SPEC> {
        FTFIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable bit for channel half transfer finish interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn htfie(&mut self) -> HTFIE_W<CH0CTL_SPEC> {
        HTFIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable bit for channel error interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<CH0CTL_SPEC> {
        ERRIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Transfer direction"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<CH0CTL_SPEC> {
        DIR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Circular mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmen(&mut self) -> CMEN_W<CH0CTL_SPEC> {
        CMEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Next address generation algorithm of peripheral"]
    #[inline(always)]
    #[must_use]
    pub fn pnaga(&mut self) -> PNAGA_W<CH0CTL_SPEC> {
        PNAGA_W::new(self, 6)
    }
    #[doc = "Bit 7 - Next address generation algorithm of memory"]
    #[inline(always)]
    #[must_use]
    pub fn mnaga(&mut self) -> MNAGA_W<CH0CTL_SPEC> {
        MNAGA_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - Transfer data size of peripheral"]
    #[inline(always)]
    #[must_use]
    pub fn pwidth(&mut self) -> PWIDTH_W<CH0CTL_SPEC> {
        PWIDTH_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Transfer data size of memory"]
    #[inline(always)]
    #[must_use]
    pub fn mwidth(&mut self) -> MWIDTH_W<CH0CTL_SPEC> {
        MWIDTH_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Priority level"]
    #[inline(always)]
    #[must_use]
    pub fn prio(&mut self) -> PRIO_W<CH0CTL_SPEC> {
        PRIO_W::new(self, 12)
    }
    #[doc = "Bit 14 - Memory to Memory Mode"]
    #[inline(always)]
    #[must_use]
    pub fn m2m(&mut self) -> M2M_W<CH0CTL_SPEC> {
        M2M_W::new(self, 14)
    }
}
#[doc = "Channel 0 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH0CTL_SPEC;
impl crate::RegisterSpec for CH0CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch0ctl::R`](R) reader structure"]
impl crate::Readable for CH0CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch0ctl::W`](W) writer structure"]
impl crate::Writable for CH0CTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH0CTL to value 0"]
impl crate::Resettable for CH0CTL_SPEC {
    const RESET_VALUE: u32 = 0;
}
