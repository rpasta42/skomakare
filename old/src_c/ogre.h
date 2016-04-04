#ifndef OGRE_H_INCLUDED
#define OGRE_H_INCLUDED

#define __cplusplus
//#define skmk_ogre_single_thread

#ifndef __cplusplus
  #include "utils.h"
#else
  #include <string>

  #include <OgreCamera.h>
  #include <OgreEntity.h>
  #include <OgreLogManager.h>
  #include <OgreOverlay.h>
  #include <OgreOverlayElement.h>
  #include <OgreOverlayManager.h>
  #include <OgreRoot.h>
  #include <OgreViewport.h>
  #include <OgreSceneManager.h>
  #include <OgreRenderWindow.h>
  #include <OgreConfigFile.h>

  #include <OISEvents.h>
  #include <OISInputManager.h>
  #include <OISKeyboard.h>
  #include <OISMouse.h>

  #include <SdkTrays.h>

extern "C" {
  #include "utils.h"
}


class OgreFramework
  : public Ogre::Singleton<OgreFramework>, OIS::KeyListener, OIS::MouseListener, OgreBites::SdkTrayListener
{
public:
	OgreFramework();
	~OgreFramework();

	bool initOgre(Ogre::String wndTitle,
                OIS::KeyListener *pKeyListener = 0,
                OIS::MouseListener *pMouseListener = 0);

	void updateOgre(double timeSinceLastFrame);
	void moveCamera();
	void getInput();

	bool isOgreToBeShutDown() const {
	  return m_bShutDownOgre;
  }

	bool keyPressed(const OIS::KeyEvent &keyEventRef);
	bool keyReleased(const OIS::KeyEvent &keyEventRef);

	bool mouseMoved(const OIS::MouseEvent &evt);
	bool mousePressed(const OIS::MouseEvent &evt, OIS::MouseButtonID id);
	bool mouseReleased(const OIS::MouseEvent &evt, OIS::MouseButtonID id);

	Ogre::Root*           m_pRoot;
	Ogre::SceneManager*   m_pSceneMgr;
	Ogre::RenderWindow*   m_pRenderWnd;
	Ogre::Camera*         m_pCamera;
	Ogre::Viewport*       m_pViewport;
	Ogre::Log*            m_pLog;
	Ogre::Timer*          m_pTimer;

	OIS::InputManager*    m_pInputMgr;
	OIS::Keyboard*        m_pKeyboard;
	OIS::Mouse*           m_pMouse;

private:
	OgreFramework(const OgreFramework&);
	OgreFramework& operator= (const OgreFramework&);

	//kk OgreBites::SdkTrayManager* m_pTrayMgr;
  Ogre::FrameEvent m_FrameEvent;
	int m_iNumScreenShots;
	bool m_bShutDownOgre;

	Ogre::Vector3   m_TranslateVector;
	Ogre::Real      m_MoveSpeed;
	Ogre::Degree    m_RotateSpeed;
	float           m_MoveScale;
	Ogre::Degree    m_RotScale;
};

class SkmkInput : public OIS::KeyListener {
public:
  OgreFramework* ogre;

  SkmkInput(OgreFramework* _ogre)
    : ogre(_ogre)
  {}

	bool keyPressed(const OIS::KeyEvent &keyEventRef) {
    ogre->keyPressed(keyEventRef);

    if(ogre->m_pKeyboard->isKeyDown(OIS::KC_F)) {
      //outside_window = !outside_window;
       //do something
    }
    return true;
  }

	bool keyReleased(const OIS::KeyEvent &keyEventRef) {
    ogre->keyReleased(keyEventRef);

    return true;
  }
};


  #ifdef skmk_ogre_single_thread

/* If Ogre3D is not compiled with multi-threaded support,
 * we cannot call ogre rendering functions from different threads.
 * This is a dirty hack to enable multiple threads to call
 * ogre functions, by queueing calls, and executing them in
 * "main thread" loop.
 */
      #include <queue>
      #include <vector>

/* Ogre functions which cannot be called from other threads */
enum class OgreAPI {
  GET_SCENE_MGR, GET_ROOT_SCENE_NODE, CREATE_LIGHT, CREATE_ENTITY,
  CREATE_CHILD_SCHENE_NODE, ATTACH_OBJECT, SET_POSITION
};

struct OgreCall {
  OgreAPI func_name;
  vector<void*> args;
  OgreCall(OgreAPI _func_name, void* _args)
    : func_name(_func_name), args(_args)
  {}
};

struct OgreTaskQueue {
  std::queue <OgreCall> calls;
  std::queue <void*> return_value;
  /* Did the main thread loop already ran this iteration? */
  bool already_ran;

  struct spin_lock *processing_sl;

  void process() {
    if (calls.empty())
      return;
    OgreCall c = calls.pop();

    switch (c.func_name) {

    }
  }

  void add(OgreCall call) {
  }
};

extern OgreTaskQueue ogreTaskQueue;
  #endif //skmk_ogre_single_thread

#endif //__cplusplus

#ifdef __cplusplus
  #define EXTERNC extern "C"
#else
  #define EXTERNC
#endif

extern struct spin_lock *ogre_sl;

EXTERNC void* ogre_get_scene_mgr();
EXTERNC void* ogre_get_root_scene_node();
EXTERNC void* ogre_create_light(const char *name);
EXTERNC void* ogre_create_entity(const char *name, const char *mesh_path);
EXTERNC void* ogre_create_child_scene_node(void *parent, const char *name);
EXTERNC void  ogre_attach_object(void *parent, void *child);
EXTERNC void  ogre_set_position(void* obj, float a, float b, float c);



#endif // OGRE_H_INCLUDED
























