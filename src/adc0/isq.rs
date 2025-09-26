#[doc = "Register `ISQ` reader"]
pub type R = crate::R<ISQ_SPEC>;
#[doc = "Register `ISQ` writer"]
pub type W = crate::W<ISQ_SPEC>;
#[doc = "Field `ISQ0` reader - 1st conversion in inserted sequence"]
pub type ISQ0_R = crate::FieldReader;
#[doc = "Field `ISQ0` writer - 1st conversion in inserted sequence"]
pub type ISQ0_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ISQ1` reader - 2nd conversion in inserted sequence"]
pub type ISQ1_R = crate::FieldReader;
#[doc = "Field `ISQ1` writer - 2nd conversion in inserted sequence"]
pub type ISQ1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ISQ2` reader - 3rd conversion in inserted sequence"]
pub type ISQ2_R = crate::FieldReader;
#[doc = "Field `ISQ2` writer - 3rd conversion in inserted sequence"]
pub type ISQ2_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ISQ3` reader - 4th conversion in inserted sequence"]
pub type ISQ3_R = crate::FieldReader;
#[doc = "Field `ISQ3` writer - 4th conversion in inserted sequence"]
pub type ISQ3_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `IL` reader - Inserted channel group length"]
pub type IL_R = crate::FieldReader;
#[doc = "Field `IL` writer - Inserted channel group length"]
pub type IL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:4 - 1st conversion in inserted sequence"]
    #[inline(always)]
    pub fn isq0(&self) -> ISQ0_R {
        ISQ0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 2nd conversion in inserted sequence"]
    #[inline(always)]
    pub fn isq1(&self) -> ISQ1_R {
        ISQ1_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - 3rd conversion in inserted sequence"]
    #[inline(always)]
    pub fn isq2(&self) -> ISQ2_R {
        ISQ2_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - 4th conversion in inserted sequence"]
    #[inline(always)]
    pub fn isq3(&self) -> ISQ3_R {
        ISQ3_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:21 - Inserted channel group length"]
    #[inline(always)]
    pub fn il(&self) -> IL_R {
        IL_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 1st conversion in inserted sequence"]
    #[inline(always)]
    #[must_use]
    pub fn isq0(&mut self) -> ISQ0_W<ISQ_SPEC> {
        ISQ0_W::new(self, 0)
    }
    #[doc = "Bits 5:9 - 2nd conversion in inserted sequence"]
    #[inline(always)]
    #[must_use]
    pub fn isq1(&mut self) -> ISQ1_W<ISQ_SPEC> {
        ISQ1_W::new(self, 5)
    }
    #[doc = "Bits 10:14 - 3rd conversion in inserted sequence"]
    #[inline(always)]
    #[must_use]
    pub fn isq2(&mut self) -> ISQ2_W<ISQ_SPEC> {
        ISQ2_W::new(self, 10)
    }
    #[doc = "Bits 15:19 - 4th conversion in inserted sequence"]
    #[inline(always)]
    #[must_use]
    pub fn isq3(&mut self) -> ISQ3_W<ISQ_SPEC> {
        ISQ3_W::new(self, 15)
    }
    #[doc = "Bits 20:21 - Inserted channel group length"]
    #[inline(always)]
    #[must_use]
    pub fn il(&mut self) -> IL_W<ISQ_SPEC> {
        IL_W::new(self, 20)
    }
}
#[doc = "Inserted sequence register\n\nYou can [`read`](crate::Reg::read) this register and get [`isq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISQ_SPEC;
impl crate::RegisterSpec for ISQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isq::R`](R) reader structure"]
impl crate::Readable for ISQ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`isq::W`](W) writer structure"]
impl crate::Writable for ISQ_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISQ to value 0"]
impl crate::Resettable for ISQ_SPEC {
    const RESET_VALUE: u32 = 0;
}
