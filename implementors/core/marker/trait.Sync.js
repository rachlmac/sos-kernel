(function() {var implementors = {};
implementors["spin"] = ["impl&lt;T&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html' title='core::marker::Sync'>Sync</a> for <a class='struct' href='spin/struct.Mutex.html' title='spin::Mutex'>Mutex</a>&lt;T&gt;","impl&lt;T&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html' title='core::marker::Sync'>Sync</a> for <a class='struct' href='spin/struct.RwLock.html' title='spin::RwLock'>RwLock</a>&lt;T&gt;",];implementors["sos_intrusive"] = ["impl&lt;T&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html' title='core::marker::Sync'>Sync</a> for <a class='struct' href='sos_intrusive/rawlink/struct.RawLink.html' title='sos_intrusive::rawlink::RawLink'>RawLink</a>&lt;T&gt; <span class='where'>where T: <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Send.html' title='core::marker::Send'>Send</a>, T: <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html' title='core::marker::Sync'>Sync</a></span>",];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
