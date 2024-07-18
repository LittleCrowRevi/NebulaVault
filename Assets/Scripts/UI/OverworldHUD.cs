using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class OverworldHUD : MonoBehaviour
{
    public Entity observedEntity;

    public GameObject hpBar;
    
    // Start is called before the first frame update
    void Start()
    {
        hpBar.GetComponent< FlexibleUiBar >().observedEntity = observedEntity;
    }

    // Update is called once per frame
    void Update()
    {
        
    }
}
