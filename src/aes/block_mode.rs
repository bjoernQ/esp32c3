#[doc = "Register `BLOCK_MODE` reader"]
pub struct R(crate::R<BLOCK_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLOCK_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLOCK_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLOCK_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLOCK_MODE` writer"]
pub struct W(crate::W<BLOCK_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLOCK_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<BLOCK_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLOCK_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLOCK_MODE` reader - Those bits decides which block mode will be used. 0x0: ECB, 0x1: CBC, 0x2: OFB, 0x3: CTR, 0x4: CFB-8, 0x5: CFB-128, 0x6: GCM, 0x7: reserved."]
pub struct BLOCK_MODE_R(crate::FieldReader<u8, u8>);
impl BLOCK_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BLOCK_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLOCK_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLOCK_MODE` writer - Those bits decides which block mode will be used. 0x0: ECB, 0x1: CBC, 0x2: OFB, 0x3: CTR, 0x4: CFB-8, 0x5: CFB-128, 0x6: GCM, 0x7: reserved."]
pub struct BLOCK_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> BLOCK_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Those bits decides which block mode will be used. 0x0: ECB, 0x1: CBC, 0x2: OFB, 0x3: CTR, 0x4: CFB-8, 0x5: CFB-128, 0x6: GCM, 0x7: reserved."]
    #[inline(always)]
    pub fn block_mode(&self) -> BLOCK_MODE_R {
        BLOCK_MODE_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Those bits decides which block mode will be used. 0x0: ECB, 0x1: CBC, 0x2: OFB, 0x3: CTR, 0x4: CFB-8, 0x5: CFB-128, 0x6: GCM, 0x7: reserved."]
    #[inline(always)]
    pub fn block_mode(&mut self) -> BLOCK_MODE_W {
        BLOCK_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES cipher block mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [block_mode](index.html) module"]
pub struct BLOCK_MODE_SPEC;
impl crate::RegisterSpec for BLOCK_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [block_mode::R](R) reader structure"]
impl crate::Readable for BLOCK_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [block_mode::W](W) writer structure"]
impl crate::Writable for BLOCK_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLOCK_MODE to value 0"]
impl crate::Resettable for BLOCK_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
