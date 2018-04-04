#include <CoreFoundation/CoreFoundation.h>
#include <CoreServices/CoreServices.h>
#include <QuickLook/QuickLook.h>

#define BUNDLE_IDENTIFIER "de.ccl.webfun.qlgenerator"

extern OSStatus generate_thumbnail(const char * bundlePath, const char * gamePath, size_t* length, uint8 **buffer);

OSStatus GeneratePreviewForURL(void *thisInterface, QLPreviewRequestRef preview, CFURLRef url, CFStringRef contentTypeUTI, CFDictionaryRef options);
void CancelPreviewGeneration(void *thisInterface, QLPreviewRequestRef preview);

/* -----------------------------------------------------------------------------
   Generate a preview for file

   This function's job is to create preview for designated file
   ----------------------------------------------------------------------------- */

OSStatus GeneratePreviewForURL(void *thisInterface, QLPreviewRequestRef preview, CFURLRef url, CFStringRef contentTypeUTI, CFDictionaryRef options)
{
    CFStringRef path = CFURLCopyFileSystemPath(url, kCFURLPOSIXPathStyle);
    CFBundleRef bundle = CFBundleGetBundleWithIdentifier(CFSTR(BUNDLE_IDENTIFIER));
    CFURLRef bundleURL = CFBundleCopyBundleURL(bundle);
    CFStringRef bundlePath = CFURLCopyFileSystemPath(bundleURL, kCFURLPOSIXPathStyle);
    
    const char * bundle_path = CFStringGetCStringPtr(bundlePath, kCFStringEncodingUTF8);
    const char * cpath = CFStringGetCStringPtr(path, kCFStringEncodingUTF8);
    
    size_t length = 0;
    uint8 *buffer = NULL;
    OSStatus result = generate_thumbnail(bundle_path, cpath, &length, &buffer);
    
    CFDataRef imageData = CFDataCreateWithBytesNoCopy(NULL, buffer, length, NULL);
    QLPreviewRequestSetDataRepresentation(preview, imageData, kUTTypeImage, NULL);
    CFRelease(imageData);
    
    CFRelease(bundlePath);
    CFRelease(bundleURL);
    CFRelease(path);
    
    return result;
}

void CancelPreviewGeneration(void *thisInterface, QLPreviewRequestRef preview)
{
    // Implement only if supported
}
